use anyhow::Context;
use clap::CommandFactory;
use colored::Colorize;
use config::NotifierConfig;
use directories::ProjectDirs;
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
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
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
async fn main() -> anyhow::Result<()> {
    // Call fetch_latest and handle the result
    match fetch_latest().await {
        Ok(_) => {
            // Successfully fetched the latest version
        }
        Err(e) => {
            // Print the error and continue
            eprintln!("Error fetching the latest version: {}", e);
        }
    }

    // Now start the main application
    start().await
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
