use anyhow::Context;
use clap::CommandFactory;
use colored::Colorize;
use config::XSNotifySettings;
use directories::ProjectDirs;
use futures::stream::ForEach;
use iced::{
    widget::{
        button, checkbox, column, row, slider, text, text_input, Checkbox, Column, Row, Text,
        TextInput,
    },
    Application, Center, Element, Renderer, Settings, Theme,
};
use notif_handling::notification_listener;
use reqwest::Error;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};
use tokio::{
    fs::{create_dir_all, File},
    io::AsyncWriteExt,
    sync::mpsc,
};
use twelf::Layer;
use xsoverlay::xs_notify;

pub mod config;
pub mod notif_handling;
pub mod xsoverlay;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

// async fn start() -> anyhow::Result<()> {
//     /* pretty_env_logger::formatted_builder()
//     .filter_level(log::LevelFilter::Debug)
//     .init(); */
//     let matches = NotifierConfig::command().get_matches();
//     /* let project_dirs = ProjectDirs::from("dev", "Gozar Productions LLC", "XS Notify")
//         .ok_or_else(|| anyhow::anyhow!("project dir lookup failed"))?;
//     let config_file_path = project_dirs.config_dir().join("./config.toml");
//     log::info!("checking if config file exists...");
//     if !config_file_path.exists() {
//         create_dir_all(project_dirs.config_dir()).await?;
//         let mut file = File::create(config_file_path.clone()).await?;
//         file.write_all(include_bytes!("./default_config.toml"))
//             .await?;
//         log::info!("default config written to {:?}", config_file_path);
//     } */
//     /* let config = NotifierConfig::with_layers(&[
//         Layer::Toml(config_file_path),
//         Layer::Env(Some("XSNOTIF_".into())),
//         Layer::Clap(matches),
//     ])
//     .context("failed to parse config")?; */
//     let (tx, mut rx) = mpsc::unbounded_channel();
//     {
//         let config = config.clone();
//         tokio::spawn(async move {
//             loop {
//                 let res = xs_notify(&mut rx, &config.host, config.port).await;
//                 log::error!(
//                     "XSOverlay notification sender died unexpectedly: {:?}, restarting sender",
//                     res
//                 );
//             }
//         });
//     }
//     loop {
//         let res = notification_listener(&config, &tx).await;
//         log::error!("Windows notification listener died unexpectedly: {:?}", res);
//     }
// }

// Entry point of the application
#[tokio::main]
async fn main() -> iced::Result {
    // Call fetch_latest and handle the result
    /* match fetch_latest().await {
        Ok(_) => {
            // Successfully fetched the latest version
        }
        Err(e) => {
            // Print the error and continue
            eprintln!("Error fetching the latest version: {}", e);
        }
    } */

    // let interface = MyApp.view();

    // Now start the main application
    // start().await

    // let mut settings = XSNotifySettings::default();

    iced::run("XS Notify", XSNotify::update, XSNotify::view)
}


#[derive(Debug)]
struct XSNotify {
    settings: XSNotifySettings,
    current_skipped_app: String,
    running: bool,
}

impl Default for XSNotify {
    fn default() -> Self {
        let settings = XSNotifySettings::default();
        XSNotify {
            settings: settings.clone(),
            current_skipped_app: String::new(),
            running: settings.auto_run,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SetAutoRun(bool),
    ToggleRun(),

    SetPort(String),
    SetHost(String),
    SetPollingRate(String),

    SetDynamicTimeout(bool),
    SetDefaultTimeout(String),

    SetReadingSpeed(String),
    SetMinTimeout(String),
    SetMaxTimeout(String),

    SetCurrentApp(String),
    AddSkippedApp(),
    RemoveSkippedApp(String),
}

/* struct Interface {
    settings: XSNotifySettings,
    current_skipped_app: String,
} */

pub fn get_config_dir() -> anyhow::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("dev", "Gozar Productions LLC", "XS Notify")
        .ok_or_else(|| anyhow::anyhow!("project dir lookup failed"))?;

    let config_dir = project_dirs.config_dir();

    Ok(config_dir.to_path_buf())
}

pub fn get_config_file_path(config_dir: PathBuf) -> anyhow::Result<PathBuf> {
    let config_file_path = config_dir.join("config.toml");
    Ok(config_file_path)
}

impl XSNotify {
    // Save settings to a TOML file
    fn save_to_file(&self) -> anyhow::Result<()> {
        let config_dir = get_config_dir()?;
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?; // Ensure the directory exists
        }
        let config_file_path = get_config_file_path(config_dir)?;
        let toml_string = toml::to_string(&self.settings).expect("Failed to serialize settings");

        let mut file = fs::File::create(config_file_path)?;
        file.write_all(toml_string.as_bytes())?;

        Ok(())
    }

    // Update settings based on the received message
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetAutoRun(value) => {
                self.settings.auto_run = value;
            }
            Message::ToggleRun() => {
                self.running = !self.running;
                if self.running {
                    println!("Starting task");
                } else {
                    println! {"ended task"};
                }
            }
            Message::SetPort(value) => {
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    if let Ok(new_value) = value.parse::<usize>() {
                        self.settings.port = new_value;
                    }
                }
            }
            Message::SetHost(value) => {
                self.settings.host = value;
            }
            Message::SetPollingRate(value) => {
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    if let Ok(new_value) = value.parse::<u64>() {
                        self.settings.polling_rate = new_value;
                    }
                }
            }
            Message::SetDynamicTimeout(value) => {
                self.settings.dynamic_timeout = value;
            }
            Message::SetDefaultTimeout(value) => {
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    if let Ok(new_value) = value.parse::<f32>() {
                        self.settings.default_timeout = new_value;
                    }
                }
            }
            Message::SetReadingSpeed(value) => {
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    if let Ok(new_value) = value.parse::<f32>() {
                        self.settings.reading_speed = new_value;
                    }
                }
            }
            Message::SetMinTimeout(value) => {
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    if let Ok(new_value) = value.parse::<f32>() {
                        self.settings.min_timeout = new_value;
                    }
                }
            }
            Message::SetMaxTimeout(value) => {
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    if let Ok(new_value) = value.parse::<f32>() {
                        self.settings.max_timeout = new_value;
                    }
                }
            }
            Message::SetCurrentApp(value) => {
                self.current_skipped_app = value;
            }
            Message::AddSkippedApp() => {
                self.settings
                    .skipped_apps
                    .push(self.current_skipped_app.clone());
                self.current_skipped_app.clear();
            }
            Message::RemoveSkippedApp(value) => {
                self.settings.skipped_apps.retain(|x| *x != value);
            }
        }
        let _ = self.save_to_file();
    }

    // Create the user interface
    fn view(&self) -> Column<Message> {
        let autorun_checkbox: Checkbox<'_, Message> =
            checkbox("Auto-run", self.settings.auto_run).on_toggle(Message::SetAutoRun);
        let autorun = row!["Auto-run", autorun_checkbox];

        let port_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Enter a number...", &self.settings.port.to_string())
                .on_input(Message::SetPort);
        let port = row!["Port", port_input];

        let host_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("localhost", &self.settings.host).on_input(Message::SetHost);
        let host = row!["Host", host_input];

        let polling_rate_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Enter a number...", &self.settings.polling_rate.to_string())
                .on_input(Message::SetPollingRate);
        let polling_rate = row!["Polling rate", polling_rate_input];

        let dynamic_timeout_checkbox = checkbox("Dynamic Timeout", self.settings.dynamic_timeout)
            .on_toggle(Message::SetDynamicTimeout);
        let dynamic_timeout = row!["Dynamic timeout", dynamic_timeout_checkbox];

        let default_timeout_input =
            text_input("Default: 5", &self.settings.default_timeout.to_string())
                .on_input(Message::SetDefaultTimeout);
        let default_timeout = row!["Default timeout", default_timeout_input];

        let reading_speed_input =
            text_input("Default: 238", &self.settings.reading_speed.to_string())
                .on_input(Message::SetReadingSpeed);
        let reading_speed = row!["Reading speed", reading_speed_input];

        let min_timeout_input = text_input("Default: 2", &self.settings.min_timeout.to_string())
            .on_input(Message::SetMinTimeout);
        let min_timeout = row!["Minimum timeout", min_timeout_input];

        let max_timeout_input = text_input("Default: 5", &self.settings.max_timeout.to_string())
            .on_input(Message::SetMaxTimeout);
        let max_timeout = row!["Maximum timeout", max_timeout_input];

        let skipped_apps_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Type an application name...", &self.current_skipped_app)
                .on_input(Message::SetCurrentApp)
                .on_submit(Message::AddSkippedApp());
        let skipped_apps_add: Button<'_, Message> =
            button("Add").on_press(Message::AddSkippedApp());
        let skipped_apps_row2 =
            self.settings
                .skipped_apps
                .iter()
                .fold(Row::new(), |row: Row<'_, Message>, item| {
                    row.push(Text::new(item.clone()))
                        .push(button("x").on_press(Message::RemoveSkippedApp(item.clone())))
                });
        let skipped_apps_row1 = row!["Skipped apps", skipped_apps_input, skipped_apps_add];

        let button_content = if self.running { "Stop" } else { "Start" };

        let run_button: Button<'_, Message> = button(button_content).on_press(Message::ToggleRun());

        let interface = column![
            autorun,
            port,
            host,
            polling_rate,
            dynamic_timeout,
            default_timeout,
            reading_speed,
            min_timeout,
            max_timeout,
            skipped_apps_row1,
            skipped_apps_row2,
            run_button
        ];

        interface
    }
}

#[test]
fn settings_update_tests() {
    let mut xs_notify = XSNotify::default();

    // Test setting the port
    xs_notify.update(Message::SetPort(String::from("2000")));
    assert_eq!(xs_notify.settings.port, 2000);

    // Test setting the host
    xs_notify.update(Message::SetHost(String::from("testing")));
    assert_eq!(xs_notify.settings.host, String::from("testing"));

    // Test setting the polling rate
    xs_notify.update(Message::SetPollingRate(String::from("100")));
    assert_eq!(xs_notify.settings.polling_rate, 100);

    // Test setting dynamic timeout
    xs_notify.update(Message::SetDynamicTimeout(false));
    assert_eq!(xs_notify.settings.dynamic_timeout, false);

    // Test setting default timeout
    xs_notify.update(Message::SetDefaultTimeout(String::from("10")));
    assert_eq!(xs_notify.settings.default_timeout, 10.);

    // Test setting reading speed
    xs_notify.update(Message::SetReadingSpeed(String::from("200")));
    assert_eq!(xs_notify.settings.reading_speed, 200.);

    // Test setting minimum timeout
    xs_notify.update(Message::SetMinTimeout(String::from("5")));
    assert_eq!(xs_notify.settings.min_timeout, 5.);

    // Test setting maximum timeout
    xs_notify.update(Message::SetMaxTimeout(String::from("30")));
    assert_eq!(xs_notify.settings.max_timeout, 30.);

    xs_notify.update(Message::SetCurrentApp(String::from("VRCX")));
    xs_notify.update(Message::AddSkippedApp());
    xs_notify.update(Message::SetCurrentApp(String::from("Discord")));
    xs_notify.update(Message::AddSkippedApp());
    xs_notify.update(Message::RemoveSkippedApp(String::from("Discord")));

    assert_eq!(xs_notify.settings.skipped_apps, vec![String::from("VRCX")])
}

async fn fetch_latest() -> Result<(), Error> {
    // Replace with your GitHub username and repository
    let username = "Erallie";
    let repository = "xs-notify";
    let current_version = "1.0.2"; // Replace with your current version

    // Fetch the latest release from GitHub
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        username, repository
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await?
        .json::<Release>()
        .await?;

    // Compare versions
    let latest_version = &response.tag_name[1..]; // Remove the 'v' prefix
    if let Ok(latest) = Version::parse(latest_version) {
        if let Ok(current) = Version::parse(current_version) {
            if latest > current {
                let current_formatted = format!("v{}", current);
                let latest_formatted = format!("v{}", latest);
                let download_link = format!(
                    "https://github.com/{}/{}/releases/tag/v{}",
                    username, repository, latest
                );
                println!("Current version: {}\n\n{} is available: {}\nCtrl + click the following link to download it: {}\n", current_formatted.blue(), "A NEW VERSION".purple().italic(), latest_formatted.bright_blue(), download_link.bright_cyan());
            } else {
                let this_formatted = format!("v{}", current);
                println!("You are on the latest version: {}\n", this_formatted.blue());
            }
        } else {
            eprintln!("Invalid current version format: {}", current_version);
        }
    } else {
        eprintln!("Invalid latest version format: {}", latest_version);
    }

    Ok(())
}
