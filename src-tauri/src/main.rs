// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::XSNotifySettings;
use error::XSNotifyError;
use notif_handling::notification_listener;
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, MutexGuard},
};
use tauri::{
    ipc::InvokeError,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Builder, Emitter, Manager, State, Url, WindowEvent,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_log::{Target, TargetKind};
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Notify,
};
use update::{download_update, fetch_latest, open_update_link, LatestResult};
use xsoverlay::{xs_notify, XSOverlayMessage};

mod config;
mod error;
pub mod notif_handling;
pub mod update;
pub mod xsoverlay;

fn main() {
    Builder::default()
        .setup(|app| {
            let config_directory = app.path().app_config_dir().expect("Failed Getting Config Path!");
            let state = Arc::new(Mutex::new(XSNotify::initialize(config_directory)));

            app.manage(state.clone());

            let toggle_run_i = Arc::new(Mutex::new(MenuItem::with_id(
                app,
                "toggle_run",
                {
                    match state.lock().unwrap().running {
                        true => "Stop bridge",
                        false => "Start bridge",
                    }
                },
                true,
                None::<&str>,
            )?));
            let about_i = MenuItem::with_id(app, "about", "Show About Info", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Open Settings", true, None::<&str>)?;
            let console_i = MenuItem::with_id(app, "console", "Show Console", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&*toggle_run_i.lock().unwrap(), &settings_i, &console_i, &about_i, &quit_i])?;

            let state_clone = Arc::clone(&state);
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| {
                    let mut window = app.get_webview_window("main").unwrap();
                    fn get_root_url() -> PathBuf {
                        match cfg!(debug_assertions) {
                            true => return PathBuf::from_str("http://localhost:1420/").unwrap(),
                            false => return PathBuf::from_str("http://tauri.localhost/").unwrap(),
                        }
                    }
                    fn get_absolute_url<T: Into<String>>(relative_path: T) -> String {
                        let relative_path: String = relative_path.into();
                        get_root_url().join(relative_path).to_string_lossy().to_string()
                    }
                    match event.id.as_ref() {
                        "quit" => {
                            log::info!("Quit menu item was clicked");
                            app.cleanup_before_exit();
                            app.exit(0);
                        }
                        "about" => {
                            // Navigate to a specific page
                            let _ = window.navigate(Url::from_str(&get_root_url().to_string_lossy()).unwrap());
                            window.show().unwrap(); // Show the window if it's hidden
                            window.set_focus().unwrap();
                        }
                        "toggle_run" => {
                            let is_running = toggle_run_2(state.lock().unwrap()).unwrap();
                            let new_text = if is_running { "Stop bridge" } else { "Start bridge" };
                            let toggle_item = toggle_run_i.lock().unwrap();
                            toggle_item.set_text(new_text).unwrap();

                            app.emit("toggle-bridge", is_running).unwrap();
                            // Set run text
                        }
                        "settings" => {
                            // Show the window when the tray icon is clicked
                            let _ = window.navigate(Url::from_str(get_absolute_url("settings").as_str()).unwrap());
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                        "console" => {
                            // Navigate to a specific page
                            let _ = window.navigate(Url::from_str(get_absolute_url("console").as_str()).unwrap());
                            window.show().unwrap(); // Show the window if it's hidden
                            window.set_focus().unwrap();
                        }
                        _ => {
                            log::warn!("Menu item {:?} not handled", event.id);
                        }
                    }
                })
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            // Get the autostart manager
            let autostart_manager = app.autolaunch();

            if state_clone.lock().unwrap().settings.lock().unwrap().auto_launch {
                // Enable autostart
                let _ = autostart_manager.enable();
            } else {
                // Disable autostart
                let _ = autostart_manager.disable();
            }
            // Check enable state
            log::info!("Registered for autostart: {}", autostart_manager.is_enabled().unwrap());

            // Minimize the window to the system tray on startup
            let window = app.get_webview_window("main").unwrap();

            if state_clone.lock().unwrap().settings.lock().unwrap().minimize_on_start {
                window.hide().unwrap(); // Hide the window
            }

            let state_clone_2 = Arc::clone(&state_clone);
            // Handle the close event to minimize to tray
            let window_handle = window.clone();
            window.on_window_event(move |event| {
                match event {
                    WindowEvent::CloseRequested { api, .. } => {
                        if state_clone_2.lock().unwrap().settings.lock().unwrap().minimize {
                            // Prevent the window from closing
                            api.prevent_close();
                            // Hide the window instead
                            window_handle.hide().unwrap();
                        }
                    }
                    _ => {}
                }
            });

            // let state_clone_3 = Arc::clone(&state_clone_2);
            let update_window = app.get_webview_window("update").unwrap();

            let app_name = app.package_info().crate_name;
            // let app_name = "xs_notify";
            let version = app.package_info().version.clone();

            tauri::async_runtime::spawn(async move {
                let result = fetch_latest(version.to_string(), app_name.to_string()).await;
                match result {
                    Ok(message) => {
                        state_clone.lock().unwrap().latest_result = message.clone();
                        if message.is_latest {
                            let _ = update_window.close().unwrap();
                        } else {
                            update_window.show().unwrap();
                            update_window.set_focus().unwrap();
                        }
                    }
                    Err(e) => {
                        let _ = update_window.close().unwrap();
                        log::error!("Error fetching latest version: {}", e);
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app.get_webview_window("main").expect("no main window").set_focus();
        }))
        .plugin(tauri_plugin_window_state::Builder::default().skip_initial_state("update").build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]), /* arbitrary number of args to pass to your app */ // Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            get_running,
            update_settings,
            toggle_run,
            download_update,
            open_update_link
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone)]
pub struct XSNotify {
    settings: Arc<Mutex<XSNotifySettings>>,
    running: bool,
    cancel: Arc<Notify>,
    latest_result: LatestResult,
}

impl XSNotify {
    fn initialize(config_directory: PathBuf) -> Self {
        let settings = XSNotifySettings::initialize(config_directory).unwrap_or_default(); // FIXME: Handle Error Properly!
        let running = settings.auto_run;
        let cancel = run_bridge(Self {
            settings: Arc::new(Mutex::new(settings.clone())),
            running,
            cancel: Arc::new(Notify::default()),
            latest_result: LatestResult::default(),
        });
        Self {
            settings: Arc::new(Mutex::new(settings)),
            running,
            cancel,
            latest_result: LatestResult::default(),
        }
    }
}

#[tauri::command]
fn get_settings(notify: State<Arc<Mutex<XSNotify>>>) -> XSNotifySettings {
    let notify = notify.lock().unwrap();
    let clone = Arc::clone(&notify.settings);
    let settings = clone.lock().unwrap().clone();
    settings
}

#[tauri::command(rename_all = "snake_case")]
fn update_settings(settings: XSNotifySettings, notify: State<Arc<Mutex<XSNotify>>>, app: tauri::AppHandle) -> Result<(), InvokeError> {
    let mut notify = notify.lock().unwrap();
    notify.settings = Arc::new(Mutex::new(settings.clone()));

    // Get the autostart manager
    let autostart_manager = app.autolaunch();

    if settings.auto_launch {
        // Enable autostart
        let _ = autostart_manager.enable();
    } else {
        // Disable autostart
        let _ = autostart_manager.disable();
    }
    // Check enable state
    log::info!("Registered for autostart: {}", autostart_manager.is_enabled().unwrap());

    fn save_settings(app: tauri::AppHandle, settings: XSNotifySettings) -> Result<(), XSNotifyError> {
        let mut config_dir = app.path().app_config_dir().expect("Failed Getting Config Path!");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?; // Ensure the directory exists
        }
        config_dir.push("config.json");

        let json_string = serde_json::to_string(&settings).expect("Failed to serialize settings");
        // log::info!("{}", json_string);
        // let mut file = fs::File::create(config_dir)?;
        // let _write = file.write_all(json_string.as_bytes())?;
        let _ = fs::write(config_dir, json_string)?;
        Ok(())
    }

    let _save = match save_settings(app, settings) {
        Ok(_) => {
            log::info!("Settings successfully updated");
        }
        Err(e) => {
            log::error!("Failed to update settings: {}", e);
        }
    };

    Ok(())
}

#[tauri::command]
fn toggle_run(state: State<Arc<Mutex<XSNotify>>>) -> Result<bool, ()> {
    let notify = state.lock().unwrap();
    let result = toggle_run_2(notify)?;
    Ok(result)
}

fn toggle_run_2(mut notify: MutexGuard<XSNotify>) -> Result<bool, ()> {
    notify.running = !notify.running;
    notify.cancel = run_bridge(notify.clone());
    Ok(notify.running)
}

#[tauri::command]
fn get_running(state: State<Arc<Mutex<XSNotify>>>) -> bool {
    let notify = state.lock().unwrap();
    notify.running
}

fn run_bridge(notify: XSNotify) -> Arc<Notify> {
    if !notify.running {
        notify.cancel.notify_waiters();
        return Arc::new(Notify::default());
    }
    // Start the task and return a command to handle the result
    let (tx, rx) = mpsc::unbounded_channel();
    let settings = notify.settings.lock().unwrap().clone();
    let host = settings.host.clone();

    let cancel = Arc::new(Notify::new());

    let cancel_clone = Arc::clone(&cancel);
    let _ = tauri::async_runtime::spawn(run_socket(cancel_clone, rx, host.clone(), settings.port));

    let cancel_clone = Arc::clone(&cancel);
    let _ = tauri::async_runtime::spawn(run_log(cancel_clone, settings, tx));

    cancel
}

async fn run_socket(cancel: Arc<Notify>, mut rx: UnboundedReceiver<XSOverlayMessage>, host: String, port: usize) {
    loop {
        tokio::select! {
            _ = cancel.notified() => {
                log::info!("Bridge socket was closed.");
                break;
            }
            // Simulate work
            _ = async {
                let res = xs_notify(&mut rx, &host, port).await;
                log::error!(
                    "XSOverlay notification sender died unexpectedly: {:?}",
                    res
                ); //This one previously had "restarting sender"
                return;
                // log::info!("Working...");
            } => {}
        }
    }
}

async fn run_log(cancel: Arc<Notify>, settings: XSNotifySettings, mut tx: UnboundedSender<XSOverlayMessage>) {
    loop {
        tokio::select! {
            _ = cancel.notified() => {
                log::info!("Notification listener was stopped.");
                break;
            }
            // Simulate work
            _ = async {
                let res = notification_listener(&settings, &mut tx).await;
                log::error!("Windows notification listener died unexpectedly: {:?}", res); //This one did NOT previously have "restarting sender"
                return;
                // log::info!("Working...");
            } => {}
        }
    }
}
