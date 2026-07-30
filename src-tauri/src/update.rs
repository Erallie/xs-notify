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
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Clone)]
pub struct LatestResult {
    pub is_latest: bool,
    pub build_link: String,
    pub installer_link: String,
}

impl Default for LatestResult {
    fn default() -> Self {
        LatestResult {
            is_latest: true,
            build_link: String::new(),
            installer_link: String::new(),
        }
    }
}

pub async fn fetch_latest<T: Into<String>>(current_version: T, app_name: T) -> Result<LatestResult, XSNotifyError> {
    // Replace with your GitHub username and repository
    let app_name: String = app_name.into();
    let current_version: String = current_version.into();

    let username = "Erallie";
    let repository = "xs-notify";

    // Fetch the latest release from GitHub
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
                let installer_link = response
                    .assets
                    .iter()
                    .find(|asset| {
                        asset
                            .name
                            .to_ascii_lowercase()
                            .ends_with("-setup.exe")
                    })
                    .map(|asset| asset.browser_download_url.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "https://github.com/{}/{}/releases/download/v{}/installer-not-found",
                            username,
                            repository,
                            latest
                        )
                    });
                log::info!(
                    "Current version: {}\n\nA NEW VERSION is available: {}\nDownload it here: {}\n",
                    current_formatted,
                    latest_formatted,
                    download_link
                );
                return Ok(LatestResult {
                    is_latest: false,
                    build_link: download_link,
                    installer_link,
                });
            } else {
                let this_formatted = format!("v{}", current);
                log::info!("You are on the latest version: {}", this_formatted);
                return Ok(LatestResult {
                    is_latest: true,
                    build_link: String::new(),
                    installer_link: String::new(),
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
pub async fn download_update(
    state: State<'_, Arc<Mutex<XSNotify>>>,
    app: tauri::AppHandle,
) -> Result<(), XSNotifyError> {
    let latest_result = state.lock().unwrap().latest_result.clone();

    let installer_path = app
        .path()
        .temp_dir()?
        .join("xs-notify-setup.exe");

    match download(latest_result.clone(), installer_path.clone()).await {
        Ok(_) => {
            let script_path = app
                .path()
                .temp_dir()?
                .join("xs-notify-update.ps1");

            let script_content = r#"
param(
    [string]$InstallerPath
)

Start-Sleep -Seconds 2

if (-not (Test-Path $InstallerPath)) {
    exit 1
}

try {
    $installerProcess = Start-Process `
        -FilePath $InstallerPath `
        -Wait `
        -PassThru

    $exitCode = $installerProcess.ExitCode

    Remove-Item $InstallerPath -Force -ErrorAction SilentlyContinue
    Remove-Item $MyInvocation.MyCommand.Path -Force -ErrorAction SilentlyContinue

    exit $exitCode
}
catch {
    exit 1
}
"#;

            std::fs::write(&script_path, script_content)?;

            let mut powershell = Command::new("powershell");

            powershell
                .arg("-NoProfile")
                .arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-WindowStyle")
                .arg("Hidden")
                .arg("-File")
                .arg(&script_path)
                .arg("-InstallerPath")
                .arg(&installer_path);

            #[cfg(target_os = "windows")]
            powershell.creation_flags(0x08000000); // CREATE_NO_WINDOW

            powershell.spawn()?;

            app.cleanup_before_exit();
            app.exit(0);
        }
        Err(error) => {
            log::error!("Failed to download installer: {error}");

            app.emit_to(
                "update",
                "update-failed",
                latest_result.build_link,
            )?;
        }
    }

    Ok(())
}

async fn download(latest_result: LatestResult, temp_file_name: PathBuf) -> Result<(), XSNotifyError> {
    // Use tokio to download the file
    let url = latest_result.installer_link;
    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;
    let mut file: File = async_fs::File::create(temp_file_name.clone()).await?;
    file.write_all(&bytes).await?;
    Ok(())
}
