use std::{
    ffi::c_void,
    io::Cursor,
    mem::size_of,
    sync::Arc,
    time::Duration,
};
use image::{
    codecs::png::PngEncoder,
    ColorType,
    ImageEncoder,
};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedSender},
    time::sleep,
};
use unicode_segmentation::UnicodeSegmentation;
use windows::{
    core::HSTRING,
    ApplicationModel::AppDisplayInfo,
    Foundation::{Size, TypedEventHandler},
    Storage::Streams::DataReader,
    UI::Notifications::{
        KnownNotificationBindings,
        Management::{
            UserNotificationListener,
            UserNotificationListenerAccessStatus,
        },
        NotificationKinds,
        UserNotification,
        UserNotificationChangedEventArgs,
        UserNotificationChangedKind,
    },
    Win32::{
        Foundation::SIZE,
        Graphics::Gdi::{
            DeleteObject,
            GetDC,
            GetDIBits,
            GetObjectW,
            ReleaseDC,
            BITMAP,
            BITMAPINFO,
            BITMAPINFOHEADER,
            BI_RGB,
            DIB_RGB_COLORS,
        },
        System::Com::{
            CoInitializeEx,
            CoUninitialize,
            COINIT_APARTMENTTHREADED,
        },
        UI::Shell::{
            SHCreateItemFromParsingName,
            IShellItemImageFactory,
            SIIGBF_ICONONLY,
        },
    },
};
use base64::{
    engine::general_purpose::STANDARD,
    Engine as _,
};

// use crate::config::{NotificationStrategy, NotifierConfig};
use crate::error::XSNotifyError;
use crate::xsoverlay::XSOverlayMessage;
use crate::XSNotifySettings;

async fn read_logo(
    display_info: AppDisplayInfo,
) -> Result<Vec<u8>, XSNotifyError> {
    tokio::task::spawn_blocking(move || {
        read_logo_blocking(display_info)
    })
    .await
    .map_err(|error| {
        XSNotifyError::Custom(format!(
            "Application icon task failed: {}",
            error
        ))
    })?
}

fn read_logo_blocking(
    display_info: AppDisplayInfo,
) -> Result<Vec<u8>, XSNotifyError> {
    let requested_size = Size {
    Width: 256.0,
    Height: 256.0,
};
    let display_name = display_info
        .DisplayName()
        .map(|value| value.to_string())
        .unwrap_or_else(|error| {
            format!("<DisplayName failed: {:?}>", error)
        });

    let description = display_info
        .Description()
        .map(|value| value.to_string())
        .unwrap_or_else(|error| {
            format!("<Description failed: {:?}>", error)
        });

    log::info!(
        "Attempting application logo retrieval: display_name={:?}, description={:?}",
        display_name,
        description
    );

    let logo_reference = display_info
        .GetLogo(requested_size)
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                concat!(
                    "GetLogo failed. ",
                    "Display error: {}; ",
                    "Debug error: {:?}; ",
                    "HRESULT: 0x{:08X}; ",
                    "HRESULT decimal: {}; ",
                    "Windows message: {}; ",
                    "requested size: {}x{}"
                ),
                error,
                error,
                error.code().0 as u32,
                error.code().0,
                error.message(),
                requested_size.Width,
                requested_size.Height
            ))
        })?;

    let open_operation = logo_reference
        .OpenReadAsync()
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "OpenReadAsync failed to start: {}",
                error
            ))
        })?;

    let logo_stream = open_operation
        .join()
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "OpenReadAsync failed while waiting: {}",
                error
            ))
        })?;

    let stream_length = logo_stream
        .Size()
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Could not determine icon stream size: {}",
                error
            ))
        })? as usize;

    if stream_length == 0 {
        return Err(XSNotifyError::Custom(
            "Application icon stream was empty".to_string(),
        ));
    }

    let reader = DataReader::CreateDataReader(&logo_stream)
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Could not create icon data reader: {}",
                error
            ))
        })?;

    let load_operation = reader
        .LoadAsync(stream_length as u32)
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "LoadAsync failed to start: {}",
                error
            ))
        })?;

    load_operation
        .join()
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "LoadAsync failed while waiting: {}",
                error
            ))
        })?;

    let mut data = vec![0_u8; stream_length];

    reader
        .ReadBytes(&mut data)
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Could not read icon bytes: {}",
                error
            ))
        })?;

    let _ = reader.Close();
    let _ = logo_stream.Close();

    Ok(data)
}

async fn read_shell_icon(
    app_user_model_id: String,
) -> Result<Vec<u8>, XSNotifyError> {
    tokio::task::spawn_blocking(move || {
        read_shell_icon_blocking(&app_user_model_id)
    })
    .await
    .map_err(|error| {
        XSNotifyError::Custom(format!(
            "Shell icon task failed: {}",
            error
        ))
    })?
}

fn read_shell_icon_blocking(
    app_user_model_id: &str,
) -> Result<Vec<u8>, XSNotifyError> {
    unsafe {
        let result = CoInitializeEx(
            None,
            COINIT_APARTMENTTHREADED,
        );

        let should_uninitialize = result.is_ok();

        const RPC_E_CHANGED_MODE: i32 =
            0x80010106_u32 as i32;

        if result.is_err() && result.0 != RPC_E_CHANGED_MODE {
            return Err(XSNotifyError::Custom(format!(
                "Could not initialize COM: HRESULT 0x{:08X}",
                result.0 as u32
            )));
        }

        let icon_result =
            extract_shell_icon(app_user_model_id);

        if should_uninitialize {
            CoUninitialize();
        }

        icon_result
    }
}

unsafe fn extract_shell_icon(
    app_user_model_id: &str,
) -> Result<Vec<u8>, XSNotifyError> {
    let parsing_name = format!(
        "shell:AppsFolder\\{}",
        app_user_model_id
    );

    log::debug!(
        "Retrieving Shell icon from {:?}",
        parsing_name
    );

    let parsing_name = HSTRING::from(parsing_name);

    let image_factory: IShellItemImageFactory =
        SHCreateItemFromParsingName(
            &parsing_name,
            None,
        )
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Could not resolve AppsFolder item for {:?}: {}",
                app_user_model_id,
                error
            ))
        })?;

    let bitmap = image_factory
        .GetImage(
            SIZE {
                cx: 256,
                cy: 256,
            },
            SIIGBF_ICONONLY,
        )
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Shell GetImage failed for {:?}: {}",
                app_user_model_id,
                error
            ))
        })?;

    let result = hbitmap_to_png(bitmap);

    let _ = DeleteObject(bitmap.into());

    result
}

unsafe fn hbitmap_to_png(
    bitmap_handle: windows::Win32::Graphics::Gdi::HBITMAP,
) -> Result<Vec<u8>, XSNotifyError> {
    let mut bitmap = BITMAP::default();

    let object_result = GetObjectW(
        bitmap_handle.into(),
        size_of::<BITMAP>() as i32,
        Some(
            &mut bitmap as *mut BITMAP
                as *mut c_void
        ),
    );

    if object_result == 0 {
        return Err(XSNotifyError::Custom(
            "GetObjectW could not read the Shell bitmap"
                .to_string(),
        ));
    }

    let width = bitmap.bmWidth;
    let height = bitmap.bmHeight.abs();

    if width <= 0 || height <= 0 {
        return Err(XSNotifyError::Custom(format!(
            "Shell returned an invalid bitmap size: {}x{}",
            width,
            height
        )));
    }

    let width_u32 = width as u32;
    let height_u32 = height as u32;

    let mut bitmap_info = BITMAPINFO::default();

    bitmap_info.bmiHeader = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width,
        // A negative height requests top-down pixel ordering.
        biHeight: -height,
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0 as u32,
        biSizeImage: width_u32
            .saturating_mul(height_u32)
            .saturating_mul(4),
        ..Default::default()
    };

    let byte_count = width_u32
        .checked_mul(height_u32)
        .and_then(|value| value.checked_mul(4))
        .ok_or_else(|| {
            XSNotifyError::Custom(
                "Shell bitmap dimensions were too large"
                    .to_string(),
            )
        })? as usize;

    let mut bgra_pixels = vec![0_u8; byte_count];

    let screen_dc = GetDC(None);

    if screen_dc.0.is_null() {
        return Err(XSNotifyError::Custom(
            "GetDC failed while reading the Shell bitmap"
                .to_string(),
        ));
    }

    let scan_lines = GetDIBits(
        screen_dc,
        bitmap_handle,
        0,
        height_u32,
        Some(bgra_pixels.as_mut_ptr() as *mut c_void),
        &mut bitmap_info,
        DIB_RGB_COLORS,
    );

    ReleaseDC(None, screen_dc);

    if scan_lines == 0 {
        return Err(XSNotifyError::Custom(
            "GetDIBits failed while reading the Shell bitmap"
                .to_string(),
        ));
    }

    // GetDIBits returns BGRA. The image encoder expects RGBA.
    for pixel in bgra_pixels.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }

    let mut png_bytes = Vec::new();

    {
        let encoder = PngEncoder::new(
            Cursor::new(&mut png_bytes)
        );

        encoder
            .write_image(
                &bgra_pixels,
                width_u32,
                height_u32,
                ColorType::Rgba8.into(),
            )
            .map_err(|error| {
                XSNotifyError::Custom(format!(
                    "Could not encode Shell icon as PNG: {}",
                    error
                ))
            })?;
    }

    Ok(png_bytes)
}

/* pub async fn get_icon(app_name: &str) -> String {
    let relative_directory = "logos";
    let mut path = PathBuf::from(relative_directory);
    path.push(format!("{}.png", app_name));

    if path.exists() {
        log::info!("got here");
        return format!("logos/{}.png", app_name);
    } else {
        return "default".to_string();
    }
} */

fn get_app_name(notif: &UserNotification) -> Result<String, XSNotifyError> {
    let app_info = notif.AppInfo()?;
    let display_info = app_info.DisplayInfo()?;
    let app_name = display_info.DisplayName()?.to_string();
    Ok(app_name)
}

fn get_app_user_model_id(
    notif: &UserNotification,
) -> Result<String, XSNotifyError> {
    let app_info = notif
        .AppInfo()
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Could not retrieve AppInfo: {}",
                error
            ))
        })?;

    let app_user_model_id = app_info
        .AppUserModelId()
        .map_err(|error| {
            XSNotifyError::Custom(format!(
                "Could not retrieve AppUserModelId: {}",
                error
            ))
        })?
        .to_string();

    if app_user_model_id.trim().is_empty() {
        return Err(XSNotifyError::Custom(
            "Application has an empty AppUserModelId".to_string(),
        ));
    }

    Ok(app_user_model_id)
}

pub async fn notif_to_message(
    notif: Arc<UserNotification>,
    config: &XSNotifySettings,
    app_name: String,
    // max_characters: usize,
) -> Result<XSOverlayMessage, XSNotifyError> {
    // log::info!("App: {}", app_name);
    /* let icon = read_logo(display_info)
    .await
    .map(encode)
    .unwrap_or_else(|err| {
        log::info!("{:?}", err.context("failed to read logo"));
        "default".to_string()
    }); */

    let app_user_model_id =
        get_app_user_model_id(&notif).ok();

    let shell_icon = match app_user_model_id {
        Some(app_user_model_id) => {
            match read_shell_icon(
                app_user_model_id.clone()
            )
            .await
            {
                Ok(icon_bytes) => {
                    log::debug!(
                        "Successfully retrieved Shell icon for {} using AUMID {:?}",
                        app_name,
                        app_user_model_id
                    );

                    Some(STANDARD.encode(icon_bytes))
                }

                Err(error) => {
                    log::warn!(
                        "Could not retrieve Shell icon for {} using AUMID {:?}: {}",
                        app_name,
                        app_user_model_id,
                        error
                    );

                    None
                }
            }
        }

        None => {
            log::warn!(
                "{} did not provide a usable AppUserModelId",
                app_name
            );

            None
        }
    };

    let icon = match shell_icon {
        Some(icon) => icon,

        None => {
            match notif
                .AppInfo()
                .and_then(|app_info| {
                    app_info.DisplayInfo()
                })
            {
                Ok(display_info) => {
                    match read_logo(display_info).await {
                        Ok(icon_bytes) => {
                            log::debug!(
                                "Successfully retrieved AppDisplayInfo icon for {}",
                                app_name
                            );

                            STANDARD.encode(icon_bytes)
                        }

                        Err(error) => {
                            log::warn!(
                                "Could not retrieve any application icon for {}: {}",
                                app_name,
                                error
                            );

                            "default".to_string()
                        }
                    }
                }

                Err(error) => {
                    log::warn!(
                        "Could not retrieve display information for {}: {}",
                        app_name,
                        error
                    );

                    "default".to_string()
                }
            }
        }
    };
    // let icon = get_icon(&app_name).await;
    let toast_binding = notif.Notification()?.Visual()?.GetBinding(&KnownNotificationBindings::ToastGeneric()?)?;
    log::debug!("Successfully retrieved toast_binding");
    // log::debug!("toast_binding.Template() = {:?}", toast_binding.Template());
    // log::debug!(
    //     "toast_binding.Hints() = {:?}",
    //     toast_binding
    //         .Hints()?
    //         .into_iter()
    //         .map(|entry| (entry.Key(), entry.Value()))
    //         .collect::<Vec<_>>()
    // );
    let text_elements = toast_binding.GetTextElements()?;
    log::debug!("Successfully retrieved text_elements");
    let title = text_elements.GetAt(0)?.Text()?.to_string();
    let mut content: String = text_elements
        .into_iter()
        .skip(1)
        .map(|element| element.Text())
        .filter_map(|el| el.ok())
        .fold(String::new(), |a, b| a + &b.to_string() + "\n");
    let mut initial_lines: Vec<&str> = content.lines().collect();
    let lines: Vec<String> = initial_lines
        .iter()
        .map(|line| {
            // Process each line into chunks of 50 graphemes
            line.graphemes(true)
                .collect::<Vec<_>>()
                .chunks(50)
                .map(|chunk| chunk.join(""))
                .collect::<Vec<String>>()
        })
        .flat_map(|chunked_lines| chunked_lines)
        .collect();
    let mut line_count = lines.len();
    if line_count > 8 {
        let line_to_cut = lines.get(8).unwrap();
        if line_count == 9 {
            // log::info!("Line Count = 9");
            let initial_len = content.len();
            content = content[..(initial_len - line_to_cut.len())].to_string();
        } else if let Some(index) = content.rfind(line_to_cut) {
            content = content[..index].to_string();
        }

        if let Some(index_2) = content.rfind(" ") {
            content = content[..index_2].to_string();
        }
        let punctuation_marks = ['.', '!', '?', ',', ';', ':', '-', '(', ')', '[', ']', '{', '}', '\"', '\''];
        let end = content.chars().count() - 1;
        let mut length_to_delete = 0;
        while punctuation_marks.contains(&content.chars().nth(end - length_to_delete).unwrap()) {
            length_to_delete += 1;
        }

        content = content[..(content.len() - length_to_delete)].to_string() + "...";
        line_count = 8;
        initial_lines = content.lines().collect();
    }

    // log::info!("Lines: {}", line_count);
    let height = ((line_count as f32) * 20 as f32) + 80 as f32;
    let words =
        initial_lines.iter().flat_map(|line| line.split_whitespace()).count() as f32 + title.lines().flat_map(|line| line.split_whitespace()).count() as f32;
    // log::info!("Word count: {}", words);
    let mut timeout = config.default_timeout;
    if config.dynamic_timeout {
        let first_timeout = words / config.reading_speed * 60 as f32;
        timeout = f32::min(f32::max(first_timeout, config.min_timeout), config.max_timeout);
    }
    
    let use_base64_icon = icon != "default";

    Ok(XSOverlayMessage {
        messageType: 1,
        index: 0,
        timeout,
        height,
        opacity: 1.,
        volume: 0.7,
        audioPath: "default".to_string(),
        title,
        content,
        useBase64Icon: use_base64_icon,
        icon,
        sourceApp: app_name,
    })
}

pub async fn polling_notification_handler(
    listener: UserNotificationListener,
    tx: &UnboundedSender<XSOverlayMessage>,
    config: &XSNotifySettings,
    // max_characters: usize,
) -> Result<(), XSNotifyError> {
    let mut prev_notifs: Option<Vec<UserNotification>> = None;
    loop {
        let notifs: Vec<Arc<UserNotification>> = listener
            .GetNotificationsAsync(NotificationKinds::Toast)?
            .await?
            .into_iter()
            .map(|notif| Arc::new(notif))
            .collect();
        if let Some(prev_notifs) = prev_notifs {
            for notif in notifs.clone().into_iter().filter(|notif| {
                prev_notifs
                    .iter()
                    .find(|prev_notif| notif.Id().unwrap_or_default() == prev_notif.Id().unwrap_or_default())
                    .is_none()
            }) {
                log::info!("Handling new notification");

                let app_name = match get_app_name(&notif) {
                    Ok(name) => name, // If successful, use the returned name
                    Err(_) => {
                        log::warn!("Could not retrive app name. Defaulting to \"Unknown Source\"");
                        "Unknown Source".to_string()
                    }
                };
                if (config.app_list.contains(&app_name) && !config.is_whitelist) || (!config.app_list.contains(&app_name) && config.is_whitelist) {
                    log::info!("Skipping notification from {}", app_name);
                } else {
                    log::debug!("Calling notif_to_message()");
                    let msg = notif_to_message(notif.clone(), config, app_name).await;
                    match msg {
                        Ok(msg) => tx.send(msg)?,
                        Err(e) => {
                            log::warn!("Failed to convert notification to XSOverlay message: {e}")
                        }
                    }
                }
            }
        }
        prev_notifs = Some(notifs.into_iter().map(|this| Arc::unwrap_or_clone(this)).collect::<Vec<UserNotification>>());
        sleep(Duration::from_millis(config.polling_rate as u64)).await;
    }
}

pub async fn listening_notification_handler(
    listener: UserNotificationListener,
    tx: &UnboundedSender<XSOverlayMessage>,
    config: &XSNotifySettings,
    // max_characters: usize,
) -> Result<(), XSNotifyError> {
    let (new_notif_tx, mut new_notif_rx) = unbounded_channel::<u32>();
    listener.NotificationChanged(
        &TypedEventHandler::new(
            move |
                _sender,
                event: windows::core::Ref<
                    '_,
                    UserNotificationChangedEventArgs,
                >,
            | {
                let Some(event) = event.as_ref() else {
                    return Ok(());
                };

                if event.ChangeKind()?
                    == UserNotificationChangedKind::Added
                {
                    log::info!(
                        "Handling new notification event"
                    );

                    let id =
                        event.UserNotificationId()?;

                    if let Err(error) =
                        new_notif_tx.send(id)
                    {
                        log::error!(
                            "Error sending ID of new notification: {}",
                            error
                        );
                    }
                }

                Ok(())
            },
        ),
    )?;
    while let Some(notif_id) = new_notif_rx.recv().await {
        if let Err(e) = async {
            let notif = listener.GetNotification(notif_id)?;

            let notif_arc = Arc::new(notif.clone());

            let app_name = match get_app_name(&notif) {
                Ok(name) => name, // If successful, use the returned name
                Err(_) => {
                    log::warn!("Could not retrive app name. Defaulting to \"Unknown Source\"");
                    "Unknown Source".to_string()
                }
            };
            if (config.app_list.contains(&app_name) && !config.is_whitelist) || (!config.app_list.contains(&app_name) && config.is_whitelist) {
                log::info!("Skipping notification from {}", app_name);
            } else {
                let msg = notif_to_message(notif_arc.clone(), config, app_name).await;
                match msg {
                    Ok(msg) => tx.send(msg)?,
                    Err(e) => log::warn!("Failed to convert notification to XSOverlay message: {e}"),
                }
            }
            Ok::<(), XSNotifyError>(())
        }
        .await
        {
            log::error!("Failed to process notification: {e}");
        };
    }
    Ok(())
}

pub async fn notification_listener(config: &XSNotifySettings, tx: &UnboundedSender<XSOverlayMessage>) -> Result<(), XSNotifyError> {
    let listener = UserNotificationListener::Current()?;
    log::info!("Requesting notification access"); //info log
    let access_status = listener.RequestAccessAsync()?.await?;
    if access_status != UserNotificationListenerAccessStatus::Allowed {
        return Err(XSNotifyError::from(format!(
            "Notification access was not granted, was instead {:?}",
            access_status
        )));
    }
    log::info!("Notification access granted"); //info
    polling_notification_handler(listener, tx, config).await
}
