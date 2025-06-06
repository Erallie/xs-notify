use std::sync::Arc;
use std::time::Duration;

use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedSender},
    time::sleep,
};
use unicode_segmentation::UnicodeSegmentation;
use windows::{
    // ApplicationModel::AppDisplayInfo,
    Foundation::TypedEventHandler, /* } */
    /*Storage::Streams::{DataReader, IRandomAccessStreamWithContentType} */
    UI::Notifications::{
        KnownNotificationBindings,
        Management::{UserNotificationListener, UserNotificationListenerAccessStatus},
        NotificationKinds, UserNotification, UserNotificationChangedEventArgs, UserNotificationChangedKind,
    },
};

// use crate::config::{NotificationStrategy, NotifierConfig};
use crate::error::XSNotifyError;
use crate::xsoverlay::XSOverlayMessage;
use crate::XSNotifySettings;

/* async fn read_logo(display_info: AppDisplayInfo) -> anyhow::Result<Vec<u8>> {
    let logo_stream = display_info
        .GetLogo(Size {
            Width: 0.,
            Height: 0.,
        })
        .context("failed to get logo with size")?
        .OpenReadAsync()
        .context("failed to open for reading")?
        .await
        .context("awaiting opening for reading failed")?;
    read_stream_to_bytes(logo_stream)
        .await
        .context("failed to read stream to bytes")
} */

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
    let icon = "default".to_string();
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
        useBase64Icon: false,
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
    listener.NotificationChanged(&TypedEventHandler::new(move |_sender, args: &Option<UserNotificationChangedEventArgs>| {
        if let Some(event) = args {
            if event.ChangeKind()? == UserNotificationChangedKind::Added {
                log::info!("Handling new notification event");
                let id = event.UserNotificationId()?;
                if let Err(e) = new_notif_tx.send(id) {
                    log::error!("Error sending ID of new notification: {e}");
                }
            };
        }
        Ok(())
    }))?;
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

/* async fn read_stream_to_bytes(
    stream: IRandomAccessStreamWithContentType,
) -> anyhow::Result<Vec<u8>> {
    let stream_len = stream.Size()? as usize;
    let mut data = vec![0u8; stream_len];
    let reader = DataReader::CreateDataReader(&stream)?;
    reader.LoadAsync(stream_len as u32)?.await?;
    reader.ReadBytes(&mut data)?;
    reader.Close().ok();
    stream.Close().ok();
    Ok(data)
} */
