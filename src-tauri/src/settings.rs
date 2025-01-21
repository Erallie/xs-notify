use crate::{error::XSNotifyError, run_bridge, XSNotify};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{ipc::InvokeError, Manager, State};
use tauri_plugin_autostart::ManagerExt;
use tokio::sync::Notify;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct XSNotifySettings {
    #[serde(default)]
    pub auto_run: bool,
    #[serde(default)]
    pub port: usize,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub polling_rate: usize,

    #[serde(default)]
    pub dynamic_timeout: bool,
    #[serde(default)]
    pub default_timeout: f32,

    #[serde(default)]
    pub reading_speed: f32,
    #[serde(default)]
    pub min_timeout: f32,
    #[serde(default)]
    pub max_timeout: f32,

    #[serde(default)]
    pub is_whitelist: bool,
    #[serde(default)]
    pub app_list: Vec<String>,

    #[serde(default)]
    pub auto_launch: bool,
    #[serde(default)]
    pub minimize: bool,
    #[serde(default)]
    pub minimize_on_start: bool,
}

impl Default for XSNotifySettings {
    fn default() -> Self {
        Self {
            auto_run: true,

            port: 42069,
            host: String::from("localhost"),
            polling_rate: 250,

            dynamic_timeout: true,
            default_timeout: 5.0,

            reading_speed: 238.0,
            min_timeout: 2.0,
            max_timeout: 120.0,

            is_whitelist: false,
            app_list: Vec::new(),

            auto_launch: true,
            minimize: true,
            minimize_on_start: false,
        }
    }
}

impl XSNotifySettings {
    pub fn initialize(mut config_directory: PathBuf) -> Result<Self, XSNotifyError> {
        if !config_directory.try_exists()? {
            std::fs::create_dir_all(&config_directory)?;
        }
        config_directory.push("config.json");

        let file_data = std::fs::read(config_directory)?;
        let settings = serde_json::from_slice::<Self>(&file_data)?;

        Ok(settings)
    }
}

#[tauri::command]
pub fn get_settings(notify: State<Arc<Mutex<XSNotify>>>) -> XSNotifySettings {
    let notify = notify.lock().unwrap();
    let clone = Arc::clone(&notify.settings);
    let settings = clone.lock().unwrap().clone();
    settings
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_settings(settings: XSNotifySettings, notify: State<Arc<Mutex<XSNotify>>>, app: tauri::AppHandle) -> Result<(), InvokeError> {
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

    notify.cancel.notify_waiters();
    notify.cancel = run_bridge(notify.clone());

    Ok(())
}
