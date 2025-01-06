use anyhow::Result;
use colored::Colorize;
use config::{get_config_dir, get_config_file_path, XSNotifySettings};
use directories::ProjectDirs;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, checkbox, column, container, row, scrollable, svg, text, text_input, toggler,
        Button, Column, Row, Svg, TextInput, Toggler,
    },
    Element,
    Length::{Fill, Shrink},
    Padding, Renderer, Size, Task, Theme,
};
use notif_handling::notification_listener;
use reqwest::Error;
use semver::Version;
use serde::Deserialize;
use std::{fs, io::Write, path::PathBuf};
use tokio::sync::mpsc;
use xsoverlay::xs_notify;

pub mod config;
pub mod notif_handling;
pub mod xsoverlay;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
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
    // iced::run("XS Notify Updater", Update::update, Update::view)
    // let _is_installed_ = install;

    // iced::run("XS Notify", XSNotify::update, XSNotify::view);
    iced::application("XS Notify", XSNotify::update, XSNotify::view)
        .window_size(Size::new(600., 800.))
        .run_with(|| {
            let mut default = XSNotify::default();
            let task = default.update(Message::Run());
            (default, task)
        })
}

#[derive(Debug, Clone)]
struct XSNotify {
    settings: XSNotifySettings,
    current_skipped_app: String,
    running: bool,
    handle: iced::task::Handle,
}

impl Default for XSNotify {
    fn default() -> Self {
        let settings = XSNotifySettings::default();
        let auto_run = settings.auto_run;
        let no_task: (Task<Message>, iced::task::Handle) = Task::none().abortable();
        XSNotify {
            settings,
            current_skipped_app: String::new(),
            running: auto_run,
            handle: no_task.1,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Run(),

    SetAutoRun(bool),
    ToggleRun(),

    TaskCompleted(()),
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
            Message::Run() => {
                if self.running {
                    // Start the task and return a command to handle the result
                    let (tx, mut rx) = mpsc::unbounded_channel();
                    let settings = self.settings.clone();
                    let host = settings.host.clone();
                    let task_socket = async move {
                        loop {
                            let res = xs_notify(&mut rx, host.clone(), settings.port.clone()).await;
                            log::error!(
                                    "XSOverlay notification sender died unexpectedly: {:?}, restarting sender",
                                    res
                                );
                        }
                    };

                    let task_log = async move {
                        loop {
                            let res = notification_listener(&settings, &tx).await;
                            log::error!(
                                "Windows notification listener died unexpectedly: {:?}",
                                res
                            );
                        }
                    };

                    let task = Task::batch(vec![
                        Task::perform(task_socket, Message::TaskCompleted),
                        Task::perform(task_log, Message::TaskCompleted),
                    ])
                    .abortable();

                    self.handle = task.1;

                    println!("Starting task");

                    return task.0;
                } else {
                    self.handle.abort();

                    println! {"ended task"};
                }
            }
            Message::SetAutoRun(value) => {
                self.settings.auto_run = value;
            }
            Message::ToggleRun() => {
                self.running = !self.running;
                return self.update(Message::Run());
            }
            Message::TaskCompleted(()) => {
                println!("Completed Task");
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
        Task::none()
    }

    // Create the user interface
    fn view(&self) -> Element<'_, Message> {
        let title_width = Fill;
        let input_width = 200;
        let title_font_size = 20;
        let description_font_size = 14;
        let setting_vertical_align = Vertical::Center;
        let setting_spacing = 8;

        let autorun_toggler: Toggler<'_, Message> =
            toggler(self.settings.auto_run).on_toggle(Message::SetAutoRun);
        let autorun_title = column![
            text("Auto-run").size(title_font_size),
            text("Automatically run on start.").size(description_font_size)
        ]
        .width(title_width);
        let autorun = row![autorun_title, autorun_toggler.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let port_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Enter a number...", &self.settings.port.to_string())
                .on_input(Message::SetPort);
        let port_title: Column<'_, Message> = column![
            text("Port").size(title_font_size),
            text("Port that XSOverlay is listening on.").size(description_font_size)
        ]
        .width(title_width);
        let port = row![port_title, port_input.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let host_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("localhost", &self.settings.host).on_input(Message::SetHost);
        let host_title: Column<'_, Message> = column![
            text("Host").size(title_font_size),
            text("The hostname that XSOverlay is listening on.").size(description_font_size)
        ]
        .width(title_width);
        let host = row![host_title, host_input.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let polling_rate_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Enter a number...", &self.settings.polling_rate.to_string())
                .on_input(Message::SetPollingRate);
        let polling_rate_title: Column<'_, Message> = column![
            text("Polling rate").size(title_font_size),
            text("The rate at which the polling strategy refreshes notifications.")
                .size(description_font_size)
        ]
        .width(title_width);
        let polling_rate = row![polling_rate_title, polling_rate_input.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let dynamic_timeout_toggler =
            toggler(self.settings.dynamic_timeout).on_toggle(Message::SetDynamicTimeout);
        let dynamic_timeout_title: Column<'_, Message> = column![
            text("Dynamic timeout").size(title_font_size),
            text("Enable this if you want the the notification display time to change depending on the amount of words being displayed.").size(description_font_size)
        ].width(title_width);
        let dynamic_timeout = row![
            dynamic_timeout_title,
            dynamic_timeout_toggler.width(input_width)
        ]
        .align_y(setting_vertical_align)
        .spacing(setting_spacing);

        let default_timeout_input =
            text_input("Default: 5", &self.settings.default_timeout.to_string())
                .on_input(Message::SetDefaultTimeout);
        let default_timeout_title: Column<'_, Message> = column![
            text("Default timeout").size(title_font_size),
            text("The duration in seconds for which notifications will remain onscreen.")
                .size(description_font_size)
        ]
        .width(title_width);
        let default_timeout = row![
            default_timeout_title,
            default_timeout_input.width(input_width)
        ]
        .align_y(setting_vertical_align)
        .spacing(setting_spacing);

        let reading_speed_input =
            text_input("Default: 238", &self.settings.reading_speed.to_string())
                .on_input(Message::SetReadingSpeed);
        let reading_speed_title: Column<'_, Message> = column![
            text("Reading speed").size(title_font_size),
            text("Your reading speed in words per minute (WPM).").size(description_font_size),
            text("The duration for which each notification remains onscreen will be calculated based off of this.")
                .size(description_font_size)
        ]
        .width(title_width);
        let reading_speed = row![reading_speed_title, reading_speed_input.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let min_timeout_input = text_input("Default: 2", &self.settings.min_timeout.to_string())
            .on_input(Message::SetMinTimeout);
        let min_timeout_title: Column<'_, Message> = column![
            text("Minimum timeout").size(title_font_size),
            text("Notifications will never display for less than this many seconds.")
                .size(description_font_size)
        ]
        .width(title_width);
        let min_timeout = row![min_timeout_title, min_timeout_input.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let max_timeout_input = text_input("Default: 5", &self.settings.max_timeout.to_string())
            .on_input(Message::SetMaxTimeout);
        let max_timeout_title: Column<'_, Message> = column![
            text("Maximum timeout").size(title_font_size),
            text("Notifications will never display for more than this many seconds.")
                .size(description_font_size)
        ]
        .width(title_width);
        let max_timeout = row![max_timeout_title, max_timeout_input.width(input_width)]
            .align_y(setting_vertical_align)
            .spacing(setting_spacing);

        let skipped_apps_input: TextInput<'_, Message, Theme, Renderer> =
            text_input("Type an application name...", &self.current_skipped_app)
                .on_input(Message::SetCurrentApp)
                .on_submit(Message::AddSkippedApp());
        let skipped_apps_title: Column<'_, Message> = column![
            text("Skipped apps").size(title_font_size),
            text("If there are apps you don't want XS Notify to push notifications for, add their names here.").size(description_font_size)
        ].width(title_width);
        // let plus_svg_handle = svg::Handle::from_path(self.images_path.join("plus.svg"));
        let plus_svg_handle = svg::Handle::from_memory(
            "<svg height=\"21\" width=\"21\" xmlns=\"http://www.w3.org/2000/svg\">
        <g stroke=\"white\" stroke-width=\"4\" stroke-linecap=\"round\">
            <line x1=\"2\" y1=\"10\" x2=\"19\" y2=\"10\" />
            <line x1=\"10\" y1=\"2\" x2=\"10\" y2=\"19\" />
        </g>
        X
    </svg>"
                .as_bytes(),
        );
        let plus_svg: Svg<'_> = svg(plus_svg_handle).width(Fill).height(Fill);
        let skipped_apps_add: Button<'_, Message> = button(plus_svg)
            .on_press(Message::AddSkippedApp())
            .height(28)
            .width(28)
            .padding(4);
        let skipped_apps_input_joined = row![skipped_apps_input, skipped_apps_add]
            .align_y(Vertical::Center)
            .spacing(6);
        let skipped_apps_row1 = row![
            skipped_apps_title,
            skipped_apps_input_joined.width(input_width)
        ]
        .align_y(setting_vertical_align)
        .spacing(setting_spacing);

        let skipped_apps_row2 = self.settings.skipped_apps.iter().fold(
            Row::new().spacing(8).align_y(setting_vertical_align),
            |row: Row<'_, Message>, item| {
                // let x_svg_handle = svg::Handle::from_path(self.images_path.join("x.svg"));
                let x_svg_handle = svg::Handle::from_memory(
                    "<svg height=\"20\" width=\"20\" xmlns=\"http://www.w3.org/2000/svg\">
            <g stroke=\"white\" stroke-width=\"4\" stroke-linecap=\"round\">
                <line x1=\"2\" y1=\"2\" x2=\"18\" y2=\"18\" />
                <line x1=\"18\" y1=\"2\" x2=\"2\" y2=\"18\" />
            </g>
            X
        </svg>"
                        .as_bytes(),
                );
                let x_svg: Svg<'_> = svg(x_svg_handle).width(12).height(12);
                row.push(
                    container(
                        row![
                            text(item.clone()),
                            button(x_svg)
                                .on_press(Message::RemoveSkippedApp(item.clone()))
                                .style(button::danger)
                                .padding(4)
                        ]
                        .spacing(4)
                        .padding(4)
                        .align_y(Vertical::Center),
                    )
                    .style(container::rounded_box),
                )
            },
        );

        let skipped_apps: Column<'_, Message> =
            column![skipped_apps_row1, skipped_apps_row2].spacing(4);

        let timeout_options: Column<'_, Message>;
        if self.settings.dynamic_timeout {
            timeout_options = column![reading_speed, min_timeout, max_timeout,]
        } else {
            timeout_options = column![default_timeout]
        }

        let button_content = if self.running { "Stop" } else { "Start" };

        let run_button: Button<'_, Message> = button(button_content).on_press(Message::ToggleRun());

        let interface = column![
            autorun,
            port,
            host,
            polling_rate,
            dynamic_timeout,
            timeout_options.spacing(8),
            skipped_apps,
            run_button
        ]
        .spacing(8)
        .padding(Padding {
            top: 12.,
            right: 16.,
            bottom: 12.,
            left: 12.,
        });

        scrollable(interface).into()
        // x_svg.into()
    }
}

#[test]
fn settings_update_tests() {
    let mut xs_notify = XSNotify::default();

    // Test setting the port
    let _ = xs_notify.update(Message::SetPort(String::from("2000")));
    assert_eq!(xs_notify.settings.port, 2000);

    // Test setting the host
    let _ = xs_notify.update(Message::SetHost(String::from("testing")));
    assert_eq!(xs_notify.settings.host, String::from("testing"));

    // Test setting the polling rate
    let _ = xs_notify.update(Message::SetPollingRate(String::from("100")));
    assert_eq!(xs_notify.settings.polling_rate, 100);

    // Test setting dynamic timeout
    let _ = xs_notify.update(Message::SetDynamicTimeout(false));
    assert_eq!(xs_notify.settings.dynamic_timeout, false);

    // Test setting default timeout
    let _ = xs_notify.update(Message::SetDefaultTimeout(String::from("10")));
    assert_eq!(xs_notify.settings.default_timeout, 10.);

    // Test setting reading speed
    let _ = xs_notify.update(Message::SetReadingSpeed(String::from("200")));
    assert_eq!(xs_notify.settings.reading_speed, 200.);

    // Test setting minimum timeout
    let _ = xs_notify.update(Message::SetMinTimeout(String::from("5")));
    assert_eq!(xs_notify.settings.min_timeout, 5.);

    // Test setting maximum timeout
    let _ = xs_notify.update(Message::SetMaxTimeout(String::from("30")));
    assert_eq!(xs_notify.settings.max_timeout, 30.);

    let _ = xs_notify.update(Message::SetCurrentApp(String::from("VRCX")));
    let _ = xs_notify.update(Message::AddSkippedApp());
    let _ = xs_notify.update(Message::SetCurrentApp(String::from("Discord")));
    let _ = xs_notify.update(Message::AddSkippedApp());
    let _ = xs_notify.update(Message::RemoveSkippedApp(String::from("Discord")));

    assert_eq!(xs_notify.settings.skipped_apps, vec![String::from("VRCX")]);
}

pub fn get_project_dirs() -> anyhow::Result<ProjectDirs> {
    let project_dirs = ProjectDirs::from("", "Gozar Productions LLC", "XS Notify")
        .ok_or_else(|| anyhow::anyhow!("project dir lookup failed"))?;

    Ok(project_dirs)
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
