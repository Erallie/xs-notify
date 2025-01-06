use crate::get_project_dirs;
use clap::{CommandFactory, Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use twelf::{config, Layer};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStrategy {
    #[default]
    Listener,
    Polling,
}

#[derive(Debug, Clone, Parser, Serialize)]
#[command(author, version, about, long_about = None)]
#[config]
pub struct XSNotifySettings {
    #[arg(short, long, default_value_t = true)]
    pub auto_run: bool,

    #[arg(short, long, default_value_t = 42069)]
    pub port: usize,
    #[arg(long, default_value = "localhost")]
    pub host: String,
    // #[arg(short, long, value_enum, default_value_t = NotificationStrategy::Listener)]
    // pub notification_strategy: NotificationStrategy,
    #[arg(long, default_value_t = 250)]
    pub polling_rate: u64,

    #[arg(short, long, default_value_t = true)]
    pub dynamic_timeout: bool,

    #[arg(long, default_value_t = 5.0)]
    pub default_timeout: f32,

    #[arg(long, default_value_t = 238.)]
    pub reading_speed: f32,

    #[arg(long, default_value_t = 2.)]
    pub min_timeout: f32,

    #[arg(long, default_value_t = 180.)]
    pub max_timeout: f32,

    // New field for skipped apps
    #[arg(long, default_values_t = Vec::<String>::new())] // Default to an empty vector
    pub skipped_apps: Vec<String>, // This will hold the array of strings
}

impl Default for XSNotifySettings {
    fn default() -> Self {
        fn load_from_file() -> anyhow::Result<XSNotifySettings> {
            let default_settings = XSNotifySettings {
                auto_run: true,
                port: 42069,
                host: String::from("localhost"),
                polling_rate: 250,
                dynamic_timeout: true,
                default_timeout: 5.0,
                reading_speed: 238.,
                min_timeout: 2.,
                max_timeout: 120.,
                skipped_apps: Vec::<String>::new(),
            };

            let config_dir = get_config_dir()?;
            if !config_dir.exists() {
                return Ok(default_settings);
            }
            let config_file_path = get_config_file_path(config_dir)?;

            if !config_file_path.exists() {
                return Ok(default_settings);
            }

            // let contents = fs::read_to_string(&config_file_path)?;
            let matches = XSNotifySettings::command().get_matches();
            let result = XSNotifySettings::with_layers(&[
                Layer::Toml(config_file_path.clone()),
                Layer::Env(Some("XSNOTIF_".into())),
                Layer::Clap(matches),
            ]);
            match result {
                Ok(settings) => Ok(settings),
                Err(_) => {
                    // If parsing fails, remove the file and write default settings
                    let _ = fs::remove_file(&config_file_path); // Ignore the result of remove_file

                    // Serialize the default settings to a string
                    let toml_string = toml::to_string(&XSNotifySettings::default())
                        .expect("Failed to serialize default settings");

                    // Create a new file and write the default settings to it
                    let mut file = fs::File::create(config_file_path)?;
                    file.write_all(toml_string.as_bytes())?;

                    // Return the default settings
                    Ok(XSNotifySettings::default())
                }
            }
        }

        load_from_file().unwrap()
    }
}

pub fn get_config_dir() -> anyhow::Result<PathBuf> {
    let project_dirs = get_project_dirs()?;

    let config_dir = project_dirs.config_dir();

    Ok(config_dir.to_path_buf())
}

pub fn get_config_file_path(config_dir: PathBuf) -> anyhow::Result<PathBuf> {
    let config_file_path = config_dir.join("config.toml");
    Ok(config_file_path)
}
