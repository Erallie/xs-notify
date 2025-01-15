use crate::error::XSNotifyError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub skipped_apps: Vec<String>,

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
            skipped_apps: Vec::new(),
            auto_launch: true,
            minimize: true,
            minimize_on_start: true, // skipped_apps: Vec::from(["App 1".to_string(), "App2".to_string()]),
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
