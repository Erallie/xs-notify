use anyhow::Context;
use clap::CommandFactory;
use colored::Colorize;
use config::NotifierConfig;
use directories::ProjectDirs;
use iced::{
    widget::{
        button, checkbox, column, row, slider, text, text_input, Column, Row, Text, TextInput,
    },
    Application, Center, Element, Renderer, Settings, Theme,
};
use notif_handling::notification_listener;
use reqwest::Error;
use semver::Version;
use serde::Deserialize;
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

async fn start() -> anyhow::Result<()> {
    /* pretty_env_logger::formatted_builder()
    .filter_level(log::LevelFilter::Debug)
    .init(); */
    let matches = NotifierConfig::command().get_matches();
    let project_dirs = ProjectDirs::from("dev", "Gozar Productions LLC", "XS Notify")
        .ok_or_else(|| anyhow::anyhow!("project dir lookup failed"))?;
    let config_file_path = project_dirs.config_dir().join("./config.toml");
    log::info!("checking if config file exists...");
    if !config_file_path.exists() {
        create_dir_all(project_dirs.config_dir()).await?;
        let mut file = File::create(config_file_path.clone()).await?;
        file.write_all(include_bytes!("./default_config.toml"))
            .await?;
        log::info!("default config written to {:?}", config_file_path);
    }
    let config = NotifierConfig::with_layers(&[
        Layer::Toml(config_file_path),
        Layer::Env(Some("XSNOTIF_".into())),
        Layer::Clap(matches),
    ])
    .context("failed to parse config")?;
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        let config = config.clone();
        tokio::spawn(async move {
            loop {
                let res = xs_notify(&mut rx, &config.host, config.port).await;
                log::error!(
                    "XSOverlay notification sender died unexpectedly: {:?}, restarting sender",
                    res
                );
            }
        });
    }
    loop {
        let res = notification_listener(&config, &tx).await;
        log::error!("Windows notification listener died unexpectedly: {:?}", res);
    }
}

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

    iced::run(
        "XS Notify",
        XSNotifySettings::update,
        XSNotifySettings::view,
    )
}

#[derive(Debug)]
struct XSNotifySettings {
    port: usize,
    host: String,
    polling_rate: u64,

    dynamic_timeout: bool,
    default_timeout: f32,

    reading_speed: f32,
    min_timeout: f32,
    max_timeout: f32,

    skipped_apps: Vec<String>,
}

impl Default for XSNotifySettings {
    fn default() -> Self {
        XSNotifySettings {
            port: 42069,
            host: String::from("localhost"),
            polling_rate: 250,
            dynamic_timeout: true,
            default_timeout: 5.0,
            reading_speed: 238.,
            min_timeout: 2.,
            max_timeout: 120.,
            skipped_apps: Vec::<String>::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SetPort(String),
    SetHost(String),
    SetPollingRate(String),

    SetDynamicTimeout(bool),
    SetDefaultTimeout(String),

    SetReadingSpeed(String),
    SetMinTimeout(String),
    SetMaxTimeout(String),
    // AddSkippedApp(String),
    // RemoveSkippedApp(String),
}

/* struct Interface {
    settings: XSNotifySettings,
    current_skipped_app: String,
} */

impl XSNotifySettings {
    /* type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = (); */

    /* fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            MyApp {
                settings: Settings::default(),
                current_skipped_app: String::new(),
            },
            Command::none(),
        )
    } */

    /* fn title(&self) -> String {
        String::from("XS Notify")
    } */

    fn update(&mut self, message: Message) {
        match message {
            Message::SetPort(value) => {
                // Allow only digits and empty input
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    // Attempt to convert the string to usize
                    match value.parse::<usize>() {
                        Ok(new_value) => {
                            self.port = new_value;
                        }
                        Err(e) => {
                            println!("Failed to convert to usize: {}", e);
                        }
                    }
                }
            }
            Message::SetHost(value) => {
                self.host = value;
            }
            Message::SetPollingRate(value) => {
                // Allow only digits and empty input
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    // Attempt to convert the string to usize
                    match value.parse::<u64>() {
                        Ok(new_value) => {
                            self.polling_rate = new_value;
                        }
                        Err(e) => {
                            println!("Failed to convert to u64: {}", e);
                        }
                    }
                }
            }
            Message::SetDynamicTimeout(value) => {
                self.dynamic_timeout = value;
            }
            Message::SetDefaultTimeout(value) => {
                // Allow only digits and empty input
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    // Attempt to convert the string to usize
                    match value.parse::<f32>() {
                        Ok(new_value) => {
                            self.default_timeout = new_value;
                        }
                        Err(e) => {
                            println!("Failed to convert to f32: {}", e);
                        }
                    }
                }
            }
            Message::SetReadingSpeed(value) => {
                // Allow only digits and empty input
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    // Attempt to convert the string to usize
                    match value.parse::<f32>() {
                        Ok(new_value) => {
                            self.reading_speed = new_value;
                        }
                        Err(e) => {
                            println!("Failed to convert to f32: {}", e);
                        }
                    }
                }
            }
            Message::SetMinTimeout(value) => {
                // Allow only digits and empty input
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    // Attempt to convert the string to usize
                    match value.parse::<f32>() {
                        Ok(new_value) => {
                            self.min_timeout = new_value;
                        }
                        Err(e) => {
                            println!("Failed to convert to f32: {}", e);
                        }
                    }
                }
            }
            Message::SetMaxTimeout(value) => {
                // Allow only digits and empty input
                if value.is_empty() || value.chars().all(char::is_numeric) {
                    // Attempt to convert the string to usize
                    match value.parse::<f32>() {
                        Ok(new_value) => {
                            self.max_timeout = new_value;
                        }
                        Err(e) => {
                            println!("Failed to convert to f32: {}", e);
                        }
                    }
                }
            } /* Message::AddSkippedApp(value) => {
                  self.settings.skipped_apps.push(value);
              }
              Message::RemoveSkippedApp(value) => {
                  // self.settings.skipped_apps.retain(|&x| x != value);
              } */
        }
    }

    fn view(&self) -> Column<Message> {
        let port_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Enter a number...", &self.port.to_string()).on_input(Message::SetPort);

        let port = row!["Port", port_input];

        let host_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("localhost", &self.host).on_input(Message::SetHost);

        let host = row!["Host", host_input];

        let polling_rate_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Enter a number...", &self.polling_rate.to_string())
                .on_input(Message::SetPollingRate);

        let polling_rate = row!["Polling rate", polling_rate_input];

        let dynamic_timeout_checkbox =
            checkbox("Dynamic Timeout", self.dynamic_timeout).on_toggle(Message::SetDynamicTimeout);

        let dynamic_timeout = row!["Dynamic timeout", dynamic_timeout_checkbox];

        let default_timeout_slider = text_input("Default: 5", &self.default_timeout.to_string())
            .on_input(Message::SetDefaultTimeout);

        let default_timeout = row!["Default timeout", default_timeout_slider];

        let reading_speed_slider = text_input("Default: 238", &self.reading_speed.to_string())
            .on_input(Message::SetReadingSpeed);

        let reading_speed = row!["Reading speed", reading_speed_slider];

        let min_timeout_slider = text_input("Default: 2", &self.min_timeout.to_string())
            .on_input(Message::SetMinTimeout);

        let min_timeout = row!["Minimum timeout", min_timeout_slider];

        let max_timeout_slider = text_input("Default: 5", &self.max_timeout.to_string())
            .on_input(Message::SetMaxTimeout);

        let max_timeout = row!["Maximum timeout", max_timeout_slider];

        /* let skipped_apps_input =
            text_input("Type an application name...", &self.current_skipped_app)
                .on_submit(Message::AddSkippedApp);

        let skipped_apps_row = Row::new().push(Text::new("Items:")).extend(
            self.settings
                .skipped_apps
                .iter()
                .map(|item| Text::new(item.clone())),
        );

        let skipped_apps = Row::new().push(skipped_apps_input).push(skipped_apps_row); */
        let interface = column![
            port,
            host,
            polling_rate,
            dynamic_timeout,
            default_timeout,
            reading_speed,
            min_timeout,
            max_timeout
        ];

        interface
    }
}

#[test]
fn settings_update_tests() {
    let mut settings = XSNotifySettings {
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

    // Test setting the port
    settings.update(Message::SetPort(String::from("2000")));
    assert_eq!(settings.port, 2000);

    // Test setting the host
    settings.update(Message::SetHost(String::from("testing")));
    assert_eq!(settings.host, String::from("testing"));

    // Test setting the polling rate
    settings.update(Message::SetPollingRate(String::from("100")));
    assert_eq!(settings.polling_rate, 100);

    // Test setting dynamic timeout
    settings.update(Message::SetDynamicTimeout(false));
    assert_eq!(settings.dynamic_timeout, false);

    // Test setting default timeout
    settings.update(Message::SetDefaultTimeout(String::from("10")));
    assert_eq!(settings.default_timeout, 10.);

    // Test setting reading speed
    settings.update(Message::SetReadingSpeed(String::from("200")));
    assert_eq!(settings.reading_speed, 200.);

    // Test setting minimum timeout
    settings.update(Message::SetMinTimeout(String::from("5")));
    assert_eq!(settings.min_timeout, 5.);

    // Test setting maximum timeout
    settings.update(Message::SetMaxTimeout(String::from("30")));
    assert_eq!(settings.max_timeout, 30.);
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
