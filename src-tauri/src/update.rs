use std::sync::{Arc, Mutex};

use crate::{error::XSNotifyError, XSNotify};
use semver::Version;
use serde::Deserialize;
use std::{path::PathBuf, process::Command};
use tauri::{Emitter, Manager, State};
use tokio::{
    fs::{self as async_fs, File},
    io::AsyncWriteExt,
};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

#[derive(Debug, Clone)]
pub struct LatestResult {
    pub is_latest: bool,
    pub build_link: String,
    pub exe_link: String,
}

impl Default for LatestResult {
    fn default() -> Self {
        LatestResult {
            is_latest: true,
            build_link: String::new(),
            exe_link: String::new(),
        }
    }
}

pub async fn fetch_latest<T: Into<String>>(current_version: T, app_name: T) -> Result<LatestResult, XSNotifyError> {
    // Replace with your GitHub username and repository
    let app_name: String = app_name.into();
    let current_version: String = current_version.into();

    let username = "Erallie";
    let repository = "xs-notify";
    let exe_filename = app_name + ".exe";

    // Fetch the latest release from GitHub
    // log::info!("Current version: {}", current_version);
    println!("Current version: {}", current_version);
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", username, repository);
    let client = reqwest::Client::new();
    let response = client.get(&url).header("User-Agent", "reqwest").send().await?.json::<Release>().await?;

    // Compare versions
    let latest_version = &response.tag_name[1..]; // Remove the 'v' prefix
    if let Ok(latest) = Version::parse(latest_version) {
        if let Ok(current) = Version::parse(current_version.as_str()) {
            if latest > current {
                let current_formatted = format!("v{}", current);
                let latest_formatted = format!("v{}", latest);
                let download_link = format!("https://github.com/{}/{}/releases/tag/v{}", username, repository, latest);
                let exe_link = format!("https://github.com/{}/{}/releases/download/v{}/{}", username, repository, latest, exe_filename);
                log::info!(
                    "Current version: {}\n\nA NEW VERSION is available: {}\nCtrl + click the following link to download it: {}\n",
                    current_formatted,
                    latest_formatted,
                    download_link
                );
                return Ok(LatestResult {
                    is_latest: false,
                    build_link: download_link,
                    exe_link,
                });
            } else {
                let this_formatted = format!("v{}", current);
                log::info!("You are on the latest version: {}", this_formatted);
                return Ok(LatestResult {
                    is_latest: true,
                    build_link: String::new(),
                    exe_link: String::new(),
                });
            }
        } else {
            log::error!("Invalid current version format: {}", current_version);
            return Err(XSNotifyError::Custom(format!("Invalid current version format: {}", current_version)));
        }
    } else {
        log::error!("Invalid latest version format: {}", latest_version);
        return Err(XSNotifyError::Custom(format!("Invalid latest version format: {}", latest_version)));
    }
}

#[tauri::command]
pub fn open_update_link(state: State<Arc<Mutex<XSNotify>>>, app: tauri::AppHandle) {
    let latest_result = state.lock().unwrap().latest_result.clone();
    if webbrowser::open(&latest_result.build_link).is_ok() {
        log::info!("Opened {} in the web browser.", &latest_result.build_link);
        app.get_webview_window("update").unwrap().close().unwrap();
    } else {
        log::error!("Failed to open the web browser.");
    }
}

#[tauri::command]
pub async fn download_update(state: State<'_, Arc<Mutex<XSNotify>>>, app: tauri::AppHandle) -> Result<(), XSNotifyError> {
    let latest_result = state.lock().unwrap().latest_result.clone();
    // Start the download in a separate task

    let app_name = app.package_info().crate_name;
    // let temp_file_name = app_name.to_owned() + ".temp";
    let temp_file_name = app.path().temp_dir()?.join(format!("{}.temp", app_name));

    match download(latest_result.clone(), temp_file_name.clone()).await {
        Ok(_) => {
            // let exe_name = app_name.to_owned() + ".exe";
            let exe_name = std::env::current_exe()?;
            // let exe_name = app.path().dir
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
            let temp_script_path = app.path().temp_dir()?.join(format!("{}-update.ps1", app_name));
            // let temp_script_path = app_name.to_owned() + "-update.ps1";
            std::fs::write(temp_script_path.clone(), script_content).expect("Failed to write xs_notify.update_program script file");

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

            app.cleanup_before_exit();
            app.exit(0);
        }
        Err(e) => {
            log::error!("Failed to download file: {e}");
            app.emit_to("update", "update-failed", latest_result.build_link).unwrap();
        }
    };
    Ok(())
}

async fn download(latest_result: LatestResult, temp_file_name: PathBuf) -> Result<(), XSNotifyError> {
    // Use tokio to download the file
    let url = latest_result.exe_link;
    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;
    let mut file: File = async_fs::File::create(temp_file_name.clone()).await?;
    file.write_all(&bytes).await?;
    Ok(())
}
