use crate::calllib::parse_network_list;
use crate::message::Message;
use crate::state::ConsoleApp;
use crate::commands::{get_interface_names, get_monitor_interfaces, run_command};
use crate::update::commands::neutrlize;
use iced::{Command};
use iced::widget::scrollable;
mod commands;


pub fn update(self_: &mut ConsoleApp, message: Message) -> Command<Message> {
    match message {
        Message::ActuallySelected(value) => {
            self_.selected_str = value.clone();
            self_.selected_n = value.parse().unwrap_or(usize::max_value());
            Command::none()
        }
        // --- Handle input changes ---

        Message::StationMacInputChanged(value) => {
            self_.station_mac = value;
            Command::none()
        }

        Message::NewMonitorInputChanged(value) => {
            self_.new_monitor_input = value;
            Command::none()
        }

 
        Message::RefreshInterfaces => {
            self_.interfaces = get_interface_names();
            Command::none()
        }
        Message::RefreshMonitorInterfaces => {
            self_.monitor_interfaces = get_monitor_interfaces();
            Command::none()
        }
        Message::InterfaceSelected(selected) => {
            self_.selected_interface = selected.clone();
            // No more interface_input field
            Command::none()
        }
        Message::MonitorSelected(selected) => {
            self_.selected_monitor = selected.clone();
            // No more monitor_input field
            Command::none()
        }
        Message::DownInterfaceSelected(selected) => {
            if let Some(ref interface) = selected {
                self_.down_interface_input = interface.clone();
            }
            Command::none()
        }
        Message::UpInterfaceSelected(selected) => {
            if let Some(ref interface) = selected {
                self_.up_interface_input = interface.clone();
            }
            Command::none()
        }
        Message::ListInterfaces => {
            self_.console_output.push_str("\n> Requesting interface list...");
            self_.is_loading = true;
            Command::perform(
                run_command("ip".to_string(), vec!["a".to_string()]),
                Message::CommandCompleted,
            )
        }
        Message::AddMonitor => {
            if self_.selected_interface.is_none() || self_.new_monitor_input.is_empty() {
                self_.console_output.push_str("\n> Error: Interface and Monitor inputs cannot be empty.");
                return Command::none();
            }
            self_.console_output.push_str("\n> Adding monitor...");
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "iw".to_string(),
                    vec![
                        "dev".to_string(),
                        self_.selected_interface.clone().unwrap(),
                        "interface".to_string(),
                        "add".to_string(),
                        self_.new_monitor_input.clone(),
                        "type".to_string(),
                        "monitor".to_string(),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        Message::DownInterface => {
            self_.console_output.push_str("\n> Downing interface...");
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "ip".to_string(),
                    vec![
                        "link".to_string(),
                        "set".to_string(),
                        self_.down_interface_input.clone(),
                        "down".to_string(),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        Message::UpInterface => {
            self_.console_output.push_str("\n> Upping interface...");
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "ip".to_string(),
                    vec![
                        "link".to_string(),
                        "set".to_string(),
                        self_.up_interface_input.clone(),
                        "up".to_string(),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        Message::KillNetworkServices => {
            self_.console_output.push_str("\n> KIlling netwok services...");
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "airmon-ng".to_string(),
                    vec!["check".to_string(), "kill".to_string()],
                ),
                Message::CommandCompleted,
            )
        }
        Message::LiftNetworkServices => {
            self_.network_services_killed = false;
            self_.console_output
                .push_str("\n> Restarting network services...");
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "systemctl".to_string(),
                    vec![
                        "restart".to_string(),
                        "NetworkManager.service".to_string(),
                        "wpa_supplicant.service".to_string(),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        Message::StartCollectingNetworkList => {
            self_.console_output.push_str(&format!(
                "\n> sudo airodump-ng {} --output-format csv -w {}",
                self_.selected_monitor.clone().unwrap_or_default(), self_.path_to_network
            ));
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "x-terminal-emulator".to_string(),
                    vec![
                        "-e".to_string(),
                        "bash".to_string(),
                        "-c".to_string(),
                        format!(
                            "sudo airodump-ng {} --output-format csv -w {}",
                            self_.selected_monitor.clone().unwrap_or_default(), self_.path_to_network
                        ),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        Message::SelectAPFile => {
            self_.console_output.push_str("\n> Opening file selection dialog for AP file...");
            let args = vec![
                "--file-selection".to_string(),
                "--title=Select Target AP File".to_string(),
                "--file-filter=*.csv *.txt".to_string(),
                "--filename=/root/.scans/".to_string(),
            ];
            self_.is_loading = true;
            Command::perform(
                run_command("zenity".to_string(), args),
                |result| match result {
                    Ok(output) => {
                        let file_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !file_path.is_empty() {
                            Message::SetPathToApFile(file_path)
                        } else {
                            Message::CommandCompleted(Ok(output))
                        }
                    }
                    Err(e) => Message::CommandCompleted(Err(e)),
                },
            )
        }

        //this just prints all APs from the file
        Message::ChooseTargetAP => {
                //pub fn new(bssid: String, first_seen: String, last_seen: String, channel: u8, speed: String, privacy: String, cipher: String, authentication: String, power: i32, beacons: u32, iv: u32, lan_ip: String, id_length: u32, essid: String, key: String)
                if self_.path_to_csv_network.is_empty() {
                    self_.console_output
                        .push_str("\n> No AP file selected. Please select a file first.");
                    return Command::none();
                }
                self_.aps = parse_network_list(self_.path_to_csv_network.clone());
                //let aps = self.aps.clone();
                for i in 0..self_.aps.len() {
                    self_.console_output.push_str(&format!(
                        "\n> {}: {}",
                        i,
                        self_.aps[i].to_string_less()
                    ));
                }

                Command::none()
            }
        Message::DeauthTarget => {
            if self_.target_ap.essid.is_empty() || self_.station_mac.len() != 17 {
                self_.console_output.push_str("\n> Not enough args");
                return Command::none();
            }
            self_.console_output.push_str(&format!(
                "sudo aireplay-ng --deauth 10 -a {} -c {} {}",
                self_.target_ap.bssid.clone(),
                self_.station_mac.clone(),
                self_.selected_monitor.clone().unwrap_or_default()
            ));
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "x-terminal-emulator".to_string(),
                    vec![
                        "-e".to_string(),
                        "bash".to_string(),
                        "-c".to_string(),
                        format!(
                            "sudo aireplay-ng --deauth 10 -a {} -c {} {}",
                            self_.target_ap.bssid.clone(),
                            self_.station_mac.clone(),
                            self_.selected_monitor.clone().unwrap_or_default()
                        ),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        Message::StartCapturing => {
            if self_.target_ap.essid.is_empty() {
                self_.console_output.push_str("\n> No target AP selected. Please select an AP first.");
                return Command::none();
            }
            self_.console_output.push_str(&format!(
                "sudo airodump-ng --bssid {} -c {} {} --output-format cap -w {}",
                self_.target_ap.bssid.clone(),
                self_.target_ap.channel.clone(),
                self_.selected_monitor.clone().unwrap_or_default(),
                self_.path_to_network.clone() + &self_.target_ap.essid.clone()
            ));
            self_.is_loading = true;
            Command::perform(
                run_command(
                    "x-terminal-emulator".to_string(),
                    vec![
                        "-e".to_string(),
                        "bash".to_string(),
                        "-c".to_string(),
                        format!(
                            "sudo airodump-ng --bssid {} -c {} {} --output-format cap -w \"{}\"",
                            self_.target_ap.bssid.clone(),
                            self_.target_ap.channel.clone(),
                            self_.selected_monitor.clone().unwrap_or_default(),
                            self_.path_to_network.clone() + &neutrlize(self_.target_ap.essid.clone())
                        ),
                    ],
                ),
                Message::CommandCompleted,
            )
        }
        // --- Handle command completion (Keep or modify as needed) ---
        Message::CommandCompleted(result) => {
            let output_text = match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    format!(
                        "Status: {}\n--- stdout ---\n{}\n--- stderr ---\n{}",
                        output.status, stdout, stderr
                    )
                }
                Err(e) => {
                    format!("Execution failed: {}", e)
                }
            };
            self_.console_output.push_str(&output_text);
            self_.console_output.push('\n');
            self_.is_loading = false;
            scrollable::snap_to(self_.scrollable_id.clone(), scrollable::RelativeOffset::END)
        }
        Message::SetPathToApFile(path) => {
            self_.path_to_csv_network = path.clone();
            self_.console_output.push_str("\n> Path to AP file set to ");
            self_.console_output.push_str(&path);
            return scrollable::snap_to(
                self_.scrollable_id.clone(),
                scrollable::RelativeOffset::END,
            );
        }
        Message::ActuallySelect => {
            self_.console_output.push_str("\n> Selected");
            if self_.selected_n == usize::max_value() || self_.selected_n > self_.aps.len() as usize {
                self_.console_output.push_str("\n> Invalid selection. Please select a valid AP.");
                return scrollable::snap_to(
                    self_.scrollable_id.clone(),
                    scrollable::RelativeOffset::END,
                );
            }
            self_.target_ap = self_.aps[self_.selected_n.clone()].clone();
            self_.console_output.push_str(&format!("\n> Selected AP: {}", self_.target_ap.essid));
            return scrollable::snap_to(
                self_.scrollable_id.clone(),
                scrollable::RelativeOffset::END,
            );
        }
        _ => Command::none(),
    }
}
