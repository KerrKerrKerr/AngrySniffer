// main.rs

mod calllib;
mod message;
mod state;
mod update;
mod ui;

use calllib::AP;
use iced::widget::Id;
use iced::{Element, Font, Size, Task};
use message::Message;
use nix::unistd::geteuid;
use state::ConsoleApp;
use std::process::exit;
use update::commands::prompt_sudo_password;

/// JetBrains Mono font referenced by name (must be installed on the system)
const JETBRAINS_MONO: Font = Font::with_name("JetBrains Mono");

#[derive(Clone)]
pub struct AppFlags {
    pub storage_location: String,
    pub remote_server_credentials: String,
    pub local_password_list: String,
    pub terminal: String,
    pub sudo_password: String,
}

fn main() -> iced::Result {
    if !geteuid().is_root() {
        eprintln!("Some features need root. You will be prompted for your sudo password.");
    }

    let sudo_password = prompt_sudo_password();
    if sudo_password.is_empty() && !geteuid().is_root() {
        eprintln!(
            "Warning: No sudo password entered. Privileged features may fail."
        );
    }

    let mut storage_location = String::new();
    let mut remote_server_credentials = String::new();
    let mut local_password_list = String::new();
    let mut terminal = String::new();
    let config_path = "./angrysniffer.toml";
    if !std::path::Path::new(config_path).exists() {
        eprintln!(
            "Configuration file {} does not exist. Creating default…",
            config_path
        );
        let default_config = r#"# AngrySniffer Configuration
[settings]
storage_location = ".scans/"
remote_server_credentials = ""
local_password_list = ""
terminal = ""
"#;
        if let Err(e) = std::fs::write(config_path, default_config) {
            eprintln!("Failed to create configuration file: {e}");
            exit(1);
        }
        eprintln!("Default configuration file created at {config_path}");
    } else {
        eprintln!("Configuration file {config_path} found.");
        match std::fs::read_to_string(config_path) {
            Ok(config_content) => match toml::from_str::<toml::Value>(&config_content) {
                Ok(config) => {
                    if let Some(settings) = config.get("settings") {
                        if let Some(s) = settings.get("storage_location").and_then(|v| v.as_str()) {
                            storage_location = s.to_string();
                        }
                        if let Some(s) = settings
                            .get("remote_server_credentials")
                            .and_then(|v| v.as_str())
                        {
                            remote_server_credentials = s.to_string();
                        }
                        if let Some(s) =
                            settings.get("local_password_list").and_then(|v| v.as_str())
                        {
                            local_password_list = s.to_string();
                        }
                        if let Some(s) = settings.get("terminal").and_then(|v| v.as_str()) {
                            terminal = s.to_string();
                        }
                    }
                    eprintln!("Configuration loaded successfully.");
                }
                Err(e) => {
                    eprintln!("Failed to parse configuration file: {e}");
                    exit(1);
                }
            },
            Err(e) => {
                eprintln!("Failed to read configuration file: {e}");
                exit(1);
            }
        }
    }

    if storage_location.is_empty() {
        storage_location = String::from(".scans/");
    }
    if !storage_location.ends_with('/') {
        storage_location.push('/');
    }

    if let Err(e) = std::fs::create_dir_all(&storage_location) {
        eprintln!(
            "Failed to create storage directory {}: {e}. Create it manually.",
            storage_location
        );
        exit(1);
    }

    let settings_at_start = AppFlags {
        storage_location: storage_location.clone(),
        remote_server_credentials: remote_server_credentials.clone(),
        local_password_list: local_password_list.clone(),
        terminal: terminal.clone(),
        sudo_password: sudo_password.clone(),
    };

    let term_display = match update::commands::resolve_terminal(&settings_at_start.terminal) {
        Ok(t) => t,
        Err(_) => {
            if settings_at_start.terminal.is_empty() {
                "(auto — none found yet)".to_string()
            } else {
                format!("{} (not found)", settings_at_start.terminal)
            }
        }
    };

    let boot_log = format!(
        "AngrySniffer ready.\n[info] storage: {}\n[info] wordlist: {}\n[info] terminal: {}\n[info] Select a monitor interface, then Collect / Capture / Crack.\n",
        settings_at_start.storage_location,
        if settings_at_start.local_password_list.is_empty() {
            "(not set — open Settings)"
        } else {
            settings_at_start.local_password_list.as_str()
        },
        term_display,
    );

    iced::application(
        move || {
            (
                ConsoleApp {
                    interfaces: update::commands::get_interface_names(),
                    monitor_interfaces: update::commands::get_monitor_interfaces(),
                    selected_interface: None,
                    selected_monitor: None,
                    station_mac: String::new(),
                    selected_str: String::new(),
                    selected_n: usize::MAX,
                    aps: Vec::new(),
                    target_ap: AP::empty(),
                    path_to_network: settings_at_start.storage_location.clone(),
                    path_to_csv_network: String::new(),
                    console_output: boot_log.clone(),
                    scrollable_id: Id::unique(),
                    is_loading: false,
                    new_monitor_input: String::new(),
                    down_interface_input: String::new(),
                    up_interface_input: String::new(),
                    network_services_killed: false,
                    show_settings: false,
                    storage_location_input: settings_at_start.storage_location.clone(),
                    remote_server_credentials_input: settings_at_start
                        .remote_server_credentials
                        .clone(),
                    local_password_list_input: settings_at_start.local_password_list.clone(),
                    storage_location: settings_at_start.storage_location.clone(),
                    remote_server_credentials: settings_at_start.remote_server_credentials.clone(),
                    local_password_list: settings_at_start.local_password_list.clone(),
                    terminal: settings_at_start.terminal.clone(),
                    terminal_input: settings_at_start.terminal.clone(),
                    cap_file_path: String::new(),
                    show_console: true,
                    sort_column: 0,
                    sort_descending: false,
                    filter_text: String::new(),
                    sudo_password: settings_at_start.sudo_password.clone(),
                    jobs: Vec::new(),
                    selected_job_id: None,
                    next_job_id: 1,
                },
                Task::none(),
            )
        },
        update,
        view,
    )
    .title("AngrySniffer")
    .default_font(JETBRAINS_MONO)
    .decorations(false)
    .window(iced::window::Settings {
        size: Size::new(1100.0, 720.0),
        min_size: Some(Size::new(960.0, 640.0)),
        decorations: false,
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
