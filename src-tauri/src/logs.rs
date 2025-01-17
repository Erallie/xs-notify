use crate::{error::XSNotifyError, XSNotify};
use chrono::{
    prelude::{DateTime, Local},
    NaiveDateTime, TimeZone,
};
use regex::Regex;
use serde::Serialize;
use std::{
    fs,
    io::{self, Read},
    path::{self, Path, PathBuf},
    sync::{Arc, Mutex},
};
use tauri::{Manager, State};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Logs {
    info: String,
    msg: String,
    info_cls: String,
    extra_classes: Option<String>,
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn load_logs(app: tauri::AppHandle) -> Result<Vec<Logs>, XSNotifyError> {
    let log_dir = app.app_handle().path().app_log_dir()?;

    let mut latest_file: Option<(PathBuf, DateTime<Local>)> = None; // Store PathBuf and DateTime

    let log_entries = fs::read_dir(log_dir)?;

    // Iterate over the entries
    for entry in log_entries {
        let entry = entry?; // Handle the Result
        let path = entry.path(); // Get the path of the entry

        // Check if it's a file and has a .log extension
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
            // Get the filename as a string
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                // Parse the filename into a NaiveDateTime
                if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(filename, "%Y-%m-%d at %H.%M.%S.log") {
                    // Convert NaiveDateTime to DateTime<Local>
                    let datetime: DateTime<Local> = Local
                        .from_local_datetime(&naive_datetime)
                        .single()
                        .expect("Failed to convert to local datetime");

                    // Update the latest file if this one is newer
                    if latest_file.is_none() || datetime > latest_file.as_ref().unwrap().1 {
                        latest_file = Some((path.clone(), datetime)); // Store full path as PathBuf and datetime
                    }
                }
            }
        }
    }

    // Output the latest file path
    if let Some((full_path, _)) = latest_file {
        // Destructure to get the path
        let contents = fs::read_to_string(&full_path)?;
        /* match contents {
            Ok(contents) => {
                println!("Got contents:");
                println!("Contents: {}", contents)
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        } */
        // log::info!("contents: {}", contents);
        return Ok(parse_logs(contents));
    } else {
        log::warn!("Cannot load logs: No log files found.");
    }
    Ok(Vec::new())
}

fn parse_logs(input: String) -> Vec<Logs> {
    // Define the regex pattern
    let re = Regex::new(r"(?m)^\[[^\[\]]+\](\[[^\[\]]+\])\[([A-Z]+)\](?:\[.+\])* (.+)$").unwrap();

    // Vector to hold the results
    let mut results: Vec<Logs> = Vec::new();

    // Iterate through all matches
    for cap in re.captures_iter(input.as_str()) {
        // Extract the first and second capturing groups
        if let (Some(info), Some(msg), Some(level)) = (cap.get(1), cap.get(3), cap.get(2)) {
            let mut info_cls = String::from("text-gray-500");
            let mut extra_classes: Option<String> = None;
            if level.as_str().to_lowercase() == "error" {
                info_cls = String::from("text-red-900");
                extra_classes = Some(String::from("bg-warning text-warning-content"));
            } else if level.as_str().to_lowercase() == "warn" {
                info_cls = String::from("text-orange-900");
                extra_classes = Some(String::from("bg-orange-400 text-warning-content"));
            }
            // Create an instance of MyStruct and push it to the results vector
            results.push(Logs {
                info: info.as_str().to_string(),
                msg: msg.as_str().to_string(),
                info_cls,
                extra_classes,
            })
        }
    }
    results
}
