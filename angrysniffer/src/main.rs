// main.rs

// Modules
mod calllib;
#[path = "update/commands.rs"]
mod commands;
mod message;
mod state;
mod update;
mod ui;

// Imports
use iced::{Font, Size, Task, Element};
use iced::widget::Id;

/// JetBrains Mono font referenced by name (must be installed on the system)
const JETBRAINS_MONO: Font = Font::with_name("JetBrains Mono");
use nix::unistd::geteuid;
use state::ConsoleApp;
use message::Message;
use std::process::exit;
use calllib::AP;

#[derive(Clone)]
pub struct AppFlags {
    pub storage_location: String,
    pub remote_server_credentials: String,
    pub local_password_list: String,
}

// Main function
fn main() -> iced::Result {
    // --- Check for root privileges ---
    if !geteuid().is_root() {
        eprintln!("This application requires root privileges to manage network interfaces.");
        eprintln!("Attempting to re-launch with sudo...");

        match std::env::current_exe() {
            Ok(exe_path) => {
                let current_args: Vec<String> = std::env::args().skip(1).collect();
                let mut command = std::process::Command::new("sudo");
                command.arg(&exe_path);
                command.args(&current_args);

                let cmd_string = format!("sudo {} {}", exe_path.display(), current_args.join(" "));
                eprintln!("Executing: {}", cmd_string);

                match command.spawn() {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(status) => {
                                exit(status.code().unwrap_or(1));
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to wait for the sudo'd process: {}. Please try running manually with sudo.",
                                    e
                                );
                                exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to execute sudo command: {}. Is 'sudo' installed and in your PATH? Please try running manually with sudo.",
                            e
                        );
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to get current executable path: {}. Cannot attempt to re-launch with sudo. Please run manually with sudo.",
                    e
                );
                exit(1);
            }
        }
    }
    let mut storage_location = String::new();
    let mut remote_server_credentials = String::new();
    let mut local_password_list = String::new();
    // Load or create configuration file
    let config_path = "./angrysniffer.toml";
    if !std::path::Path::new(config_path).exists() {
        eprintln!("Configuration file {} does not exist. Creating default configuration...", config_path);
        let default_config = r#"# AngrySniffer Configuration
    [settings]
    storage_location = "/root/.scans/"
    remote_server_credentials = ""
    local_password_list = ""
    "#;
        match std::fs::write(config_path, default_config) {
            Ok(_) => eprintln!("Default configuration file created at {}", config_path),
            Err(e) => {
                eprintln!("Failed to create configuration file: {}", e);
                exit(1);
            }
        }
    } else {
        eprintln!("Configuration file {} found.", config_path);
        // Parse the existing configuration file
        match std::fs::read_to_string(config_path) {
            Ok(config_content) => {
                match toml::from_str::<toml::Value>(&config_content) {
                    Ok(config) => {
                        if let Some(settings) = config.get("settings") {
                            if let Some(storage) = settings.get("storage_location") {
                                if let Some(storage_str) = storage.as_str() {
                                    storage_location = storage_str.to_string();
                                }
                            }
                            if let Some(credentials) = settings.get("remote_server_credentials") {
                                if let Some(credentials_str) = credentials.as_str() {
                                    remote_server_credentials = credentials_str.to_string();
                                }
                            }
                            if let Some(password_list) = settings.get("local_password_list") {
                                if let Some(password_list_str) = password_list.as_str() {
                                    local_password_list = password_list_str.to_string();
                                }
                            }
                        }
                        eprintln!("Configuration loaded successfully.");
                    }
                    Err(e) => {
                        eprintln!("Failed to parse configuration file: {}", e);
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read configuration file: {}", e);
                exit(1);
            }
        }
    }

    let scan_dir_path_str = "/root/.scans";
    let scan_dir_path = std::path::Path::new(scan_dir_path_str);

    if !scan_dir_path.exists() {
        eprintln!(
            "Directory {} does not exist. Creating it...",
            scan_dir_path_str
        );
        match std::fs::create_dir_all(scan_dir_path) {
            Ok(_) => {
                eprintln!("Directory {} created successfully.", scan_dir_path_str);
            }
            Err(e) => {
                eprintln!(
                    "Failed to create directory {}: {}. Please create it manually and ensure correct permissions.",
                    scan_dir_path_str, e
                );
                exit(1);
            }
        }
    } else {
        eprintln!("Directory {} already exists.", scan_dir_path_str);
    }

    let settings_at_start = AppFlags {
        storage_location: storage_location.clone(),
        remote_server_credentials: remote_server_credentials.clone(),
        local_password_list: local_password_list.clone(),
    };

    iced::application(
        move || {
            (
                ConsoleApp {
                    interfaces: commands::get_interface_names(),
                    monitor_interfaces: commands::get_monitor_interfaces(),
                    selected_interface: None,
                    selected_monitor: None,
                    station_mac: String::new(),
                    selected_str: String::new(),
                    selected_n: usize::max_value(),
                    aps: Vec::new(),
                    target_ap: AP::empty(),
                    path_to_network: String::from("/root/.scans/"),
                    path_to_csv_network: String::from(""),
                    console_output: String::from("Console ready."),
                    scrollable_id: Id::unique(),
                    is_loading: false,
                    new_monitor_input: String::new(),
                    down_interface_input: String::new(),
                    up_interface_input: String::new(),
                    network_services_killed: false,
                    show_settings: false,
                    settings_text: String::new(),
                    storage_location_input: settings_at_start.storage_location.clone(),
                    remote_server_credentials_input: settings_at_start.remote_server_credentials.clone(),
                    local_password_list_input: settings_at_start.local_password_list.clone(),
                    storage_location: settings_at_start.storage_location.clone(),
                    remote_server_credentials: settings_at_start.remote_server_credentials.clone(),
                    local_password_list: settings_at_start.local_password_list.clone(),
                    cap_file_path: String::new(),
                },
                Task::none(),
            )
        },
        update,
        view,
    )
    .default_font(JETBRAINS_MONO)
    .window(iced::window::Settings {
        size: Size::new(1000.0, 800.0),
        min_size: Some(Size::new(1000.0, 800.0)),
        ..iced::window::Settings::default()
    })
    .run()
}

fn update(state: &mut ConsoleApp, message: Message) -> Task<Message> {
    update::update(state, message)
}

fn view(state: &ConsoleApp) -> Element<'_, Message> {
    ui::view(state)
}