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
use iced::{Application, Command, Settings, Size, Element};
use nix::unistd::geteuid;
use state::ConsoleApp;
use message::Message;
use std::process::exit;
use iced::widget::scrollable;
use calllib::AP;


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

    let scan_dir_path_str = "/root/scan";
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

    ConsoleApp::run(Settings {
        window: iced::window::Settings {
            size: (Size::new(1000.0, 800.0)),
            min_size: Some(Size::new(1000.0, 800.0)),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

impl Application for ConsoleApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (ConsoleApp, Command<Message>) {
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
                path_to_network: String::from("/root/scan/"),
                path_to_csv_network: String::from("not entered"),
                console_output: String::from("Console ready."),
                scrollable_id: scrollable::Id::unique(),
                is_loading: false,
                interface_input: String::new(),
                monitor_input: String::new(),
                new_monitor_input: String::new(),
                down_interface_input: String::new(),
                up_interface_input: String::new(),
                network_services_killed: false,
                show_exit_dialog: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AngrySniffer Control Panel v 0.0.1")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update::update(self, message)
    }

    fn view(&self) -> Element<Message> {
        ui::view(self)
    }
}
