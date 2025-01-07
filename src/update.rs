use crate::{get_config_file_path, get_project_dirs};
use anyhow::Result;
use bytes::Bytes;
use colored::Colorize;
use iced::{
    alignment::{Horizontal, Vertical},
    executor,
    widget::{button, column, row, text, Column},
    Element, Executor, Settings, Task,
};
use reqwest::{Error, Response};
use semver::Version;
use serde::Deserialize;
use std::{fmt, fs, io, path::PathBuf, process::Command, string::FromUtf8Error};
use tokio::{
    fs::{self as async_fs, File},
    io::AsyncWriteExt,
    runtime::Runtime,
};
use webbrowser;

// Entry point of the application
/* #[tokio::main]
async fn main() -> iced::Result {
    let mut runtime: iced::Result;
    // Call fetch_latest and handle the result
    match fetch_latest().await {
        Ok(result) => match result.value {
            LatestEnum::IsLatest => {
                runtime = iced::run("XS Notify", XSNotify::update, XSNotify::view)
            }
            LatestEnum::NotLatest => {
                runtime = iced::run(
                    "XS Notify Updater",
                    update::Update::update,
                    update::Update::view,
                )
            }
            LatestEnum::InvalidCurrent => {
                runtime = iced::run("XS Notify", XSNotify::update, XSNotify::view)
            }
            LatestEnum::InvalidLatest => {
                runtime = iced::run("XS Notify", XSNotify::update, XSNotify::view)
            }
        },
        Err(e) => {
            runtime = iced::run("XS Notify", XSNotify::update, XSNotify::view);
            // Print the error and continue
            eprintln!("Error fetching the latest version: {}", e);
        }
    }

    // Start the main application
    runtime
} */

#[derive(Debug, Clone)]
pub struct Update {
    app_name: String,
    pub build_url: String,
    pub exe_url: String,
    exe_path: PathBuf,
    downloading: bool,
}

impl Default for Update {
    fn default() -> Self {
        let project_dirs = get_project_dirs().unwrap();
        let app_name = env!("CARGO_PKG_NAME").to_string();
        let exe_path = project_dirs
            .data_dir()
            .join(app_name.clone().to_owned() + ".exe");
        // let config_file_path = println!("exe_path: {}", exe_path.to_string_lossy());

        Update {
            app_name,
            build_url: String::new(),
            exe_url: String::new(),
            exe_path,
            downloading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Update,
    DownloadComplete,
    RunMain,
}

impl Update {
    /* fn title(&self) -> String {
        String::from("Update Checker")
    } */

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Update => {
                self.downloading = true;
                // Start the download in a separate task

                let this_self = self.clone();

                let temp_file_name = this_self.app_name.clone().to_owned() + ".temp";
                // Use tokio to download the file
                let download_future = async move {
                    let response: Response;
                    let url = this_self.exe_url;
                    println!("url: {}", url);
                    match reqwest::get(url).await {
                        Ok(res) => {
                            response = res;
                        }
                        Err(e) => {
                            eprintln!("Error downloading file: {}", e);
                            return Message::RunMain;
                        }
                    }
                    let bytes: Bytes;
                    match response.bytes().await {
                        Ok(b) => {
                            bytes = b;
                        }
                        Err(e) => {
                            eprintln!("Error downloading file: {}", e);
                            return Message::RunMain;
                        }
                    };
                    let mut file: File;
                    match async_fs::File::create(temp_file_name.clone()).await {
                        Ok(f) => {
                            file = f;
                        }
                        Err(e) => {
                            eprintln!("Error downloading file: {}", e);
                            return Message::RunMain;
                        }
                    };
                    file.write_all(&bytes)
                        .await
                        .expect(format!("Failed to write data to {}", temp_file_name).as_str());

                    Message::DownloadComplete
                };

                return Task::perform(download_future, |result| result);
            }
            Message::DownloadComplete => {
                let exe_name = self.app_name.clone().to_owned() + ".exe";
                let temp_file_name = self.app_name.clone().to_owned() + ".temp";
                // Hardcoded PowerShell script
                let script_content = r#"
param(
    [string]$TempFilePath,
    [string]$ExePath
)

# Get the path of the currently running script
$currentScriptPath = $MyInvocation.MyCommand.Path

Start-Sleep -Seconds 1

# Check if the file exists
if (Test-Path $TempFilePath) {
    # Remove the file
    Copy-Item $TempFilePath $ExePath
    Remove-Item $TempFilePath -Force
    Start-Process -FilePath $ExePath
    # Remove the script file
    Remove-Item -Path $currentScriptPath -Force
    exit
} else {
    Write-Host "File does not exist."
}
                "#;

                // Create a temporary PowerShell script file
                let temp_script_path = self.app_name.clone().to_owned() + "Update.ps1";
                std::fs::write(temp_script_path.clone(), script_content)
                    .expect("Failed to write update script file");

                // Execute the PowerShell script
                let _ = Command::new("powershell")
                    .arg("-ExecutionPolicy")
                    .arg("Bypass") // Bypass execution policy for the script
                    .arg("-File")
                    .arg(temp_script_path)
                    .arg("-TempFilePath")
                    .arg(temp_file_name)
                    .arg("-ExePath")
                    .arg(exe_name)
                    .spawn();

                iced::exit()
            }
            Message::RunMain => {
                /* Task::batch(vec![
                    iced::exit(),
                    iced::run("XS Notify", XSNotify::update, XSNotify::view),
                ]) */
                iced::exit()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let update_button = button("Update").on_press(Message::Update);
        let not_now_button = button("Not now").on_press(Message::RunMain);

        let options = row![not_now_button, update_button].spacing(4);

        let download_text: Column<'_, Message> = column![
            text("Download it at:").size(16),
            text(self.build_url.as_str()).size(16)
        ]
        .align_x(Horizontal::Center);

        let full_info: Column<'_, Message> =
            column![text("A new version is available!").size(20), download_text]
                .spacing(8)
                .align_x(Horizontal::Center);

        let content = if self.downloading {
            "Downloading new version...".into()
        } else {
            let column = column![full_info, options]
                .spacing(12)
                .align_x(Horizontal::Center);
            column.into()
        };

        content
    }
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[derive(Debug, Clone)]
pub struct LatestResult {
    pub value: bool,
    pub build_link: String,
    pub exe_link: String,
}

#[derive(Debug)]
pub enum LatestError {
    InvalidCurrentVersion(String),
    InvalidLatestVersion(String),
    Other(String),
    IoError(io::Error),
    Utf8Error(FromUtf8Error),
    ReqwestError(reqwest::Error),
}

// Implement the Display trait for your custom error
impl fmt::Display for LatestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LatestError::InvalidCurrentVersion(version) => {
                write!(f, "Invalid Current Version: {}", version)
            }
            LatestError::InvalidLatestVersion(version) => {
                write!(f, "Invalid Latest Version: {}", version)
            }
            LatestError::Other(msg) => write!(f, "Error: {}", msg),
            LatestError::IoError(err) => write!(f, "IO Error: {}", err),
            LatestError::Utf8Error(err) => write!(f, "UTF-8 Error: {}", err),
            LatestError::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
        }
    }
}

impl std::error::Error for LatestError {}

// Implement From for MyError to convert from io::Error
impl From<io::Error> for LatestError {
    fn from(err: io::Error) -> LatestError {
        LatestError::IoError(err)
    }
}

// Implement From for MyError to convert from FromUtf8Error
impl From<FromUtf8Error> for LatestError {
    fn from(err: FromUtf8Error) -> LatestError {
        LatestError::Utf8Error(err)
    }
}

// Implement From for MyError to convert from FromUtf8Error
impl From<reqwest::Error> for LatestError {
    fn from(err: reqwest::Error) -> LatestError {
        LatestError::ReqwestError(err)
    }
}

pub async fn fetch_latest() -> Result<LatestResult, LatestError> {
    // Replace with your GitHub username and repository
    let username = "Erallie";
    let repository = "xs-notify";
    let exe_filename = env!("CARGO_PKG_NAME").to_string() + ".exe";
    let current_version = env!("CARGO_PKG_VERSION"); // Replace with your current version

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
                let exe_link = format!(
                    "https://github.com/{}/{}/releases/download/v{}/{}",
                    username, repository, latest, exe_filename
                );
                println!("Current version: {}\n\n{} is available: {}\nCtrl + click the following link to download it: {}\n", current_formatted.blue(), "A NEW VERSION".purple().italic(), latest_formatted.bright_blue(), download_link.bright_cyan());
                return Ok(LatestResult {
                    value: false,
                    build_link: download_link,
                    exe_link,
                });
            } else {
                let this_formatted = format!("v{}", current);
                println!("You are on the latest version: {}\n", this_formatted.blue());
                return Ok(LatestResult {
                    value: true,
                    build_link: String::new(),
                    exe_link: String::new(),
                });
            }
        } else {
            eprintln!("Invalid current version format: {}", current_version);
            /* return Ok(LatestResult {
                value: LatestEnum::Error,
                download_link: String::new(),
            }); */
            return Err(LatestError::InvalidCurrentVersion(
                current_version.to_string(),
            ));
        }
    } else {
        eprintln!("Invalid latest version format: {}", latest_version);
        /* return Ok(LatestResult {
            value: LatestEnum::Error,
            download_link: String::new(),
        }); */
        return Err(LatestError::InvalidLatestVersion(
            latest_version.to_string(),
        ));
    }
}
