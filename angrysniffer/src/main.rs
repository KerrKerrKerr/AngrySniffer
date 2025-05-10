use calllib::{parse_network_list, AP};
use iced::widget::{button, column, text, container, scrollable, text_input, row}; // Added text_input, row
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Size, Theme};
// Added Alignment
use std::process::{exit, Output};
use std::sync::Arc;
use nix::unistd::geteuid;
mod calllib;

// Define the application state
struct ConsoleApp {
    selected_str: String,
    path_to_network: String,
    console_output: String,
    scrollable_id: scrollable::Id,
    is_loading: bool,
    // --- New state fields for text inputs ---
    interface_input: String,
    monitor_input: String,
    new_monitor_input: String,
    down_interface_input: String,
    up_interface_input: String, // <-- Add this line
    target_ap: AP,
    aps: Vec<AP>,
    path_to_csv_network: String,
    selected_n: usize,
    station_mac: String,
}

// Define the messages the application can react to
#[derive(Debug, Clone)]
enum Message {
    // --- Input changes ---
    InterfaceInputChanged(String),
    MonitorInputChanged(String),
    NewMonitorInputChanged(String),
    DownInterfaceInputChanged(String),
    UpInterfaceInputChanged(String), // <-- Add this line
    ActuallySelected(String),
    StationMacInputChanged(String),
    // --- Button presses ---
    ActuallySelect,
    ChooseTargetAP,
    ListInterfaces,
    SetInterface,
    SetMonitor,
    AddMonitor,
    DownInterface,
    UpInterface, // <-- Add this line
    KillNetworkServices,
    LiftNetworkServices,
    StartCollectingNetworkList,
    SelectAPFile, // <-- Add this line
    DeauthTarget, // Placeholder
    StartCapturing, // Placeholder
    SetPathToApFile(String),
    // --- Existing ---
    CommandCompleted(Result<Output, Arc<std::io::Error>>), // Keep for potential future use or remove if not needed
}

fn neutrlize(strng: String) -> String {
    strng.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
        .collect()
}

// Asynchronously run a shell command (remains the same for now)
async fn run_command(command: String, args: Vec<String>) -> Result<Output, Arc<std::io::Error>> {
    tokio::process::Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(Arc::new)
}

impl Application for ConsoleApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    // Initialize the application state
    fn new(_flags: ()) -> (ConsoleApp, Command<Message>) {
        (
            ConsoleApp {
                station_mac: String::new(),
                selected_str: String::new(),
                selected_n: usize::max_value(),
                aps: Vec::new(),
                target_ap: AP::empty() ,
                path_to_network: String::from("/root/scan/"),
                path_to_csv_network: String::from("not entered"),
                console_output: String::from("Console ready."),
                scrollable_id: scrollable::Id::unique(),
                is_loading: false,
                // --- Initialize new state ---
                interface_input: String::new(),
                monitor_input: String::new(),
                new_monitor_input: String::new(),
                down_interface_input: String::new(),
                up_interface_input: String::new(), // <-- Add this line
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AngrySniffer Control Panel v 0.0.0") // Updated title
    }

    // Handle messages and update the state
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {


            Message::ActuallySelected(value) => {
                self.selected_str = value.clone();
                self.selected_n = value.parse().unwrap_or(usize::max_value());
                Command::none()
            }
            // --- Handle input changes ---
            Message::InterfaceInputChanged(value) => {
                self.interface_input = value;
                Command::none()
            }
            Message::StationMacInputChanged(value) => {
                self.station_mac = value;
                Command::none()
            }
            Message::MonitorInputChanged(value) => {
                self.monitor_input = value;
                Command::none()
            }
            Message::NewMonitorInputChanged(value) => {
                self.new_monitor_input = value;
                Command::none()
            }
            Message::DownInterfaceInputChanged(value) => {
                self.down_interface_input = value;
                Command::none()
            }
            Message::UpInterfaceInputChanged(value) => {
                self.up_interface_input = value;
                Command::none()
            }
            // --- Handle button presses (Log only for now) ---
            Message::ListInterfaces => {
                self.console_output.push_str("\n> Requesting interface list...");
                self.is_loading = true; // Indicate loading state
                Command::perform(
                    run_command("ip".to_string(), vec!["a".to_string()]), // Command to run
                    Message::CommandCompleted // Message to send on completion
                )
            }
            Message::SetInterface => {
                self.console_output.push_str(&format!("\n> Set Interface [{}]", self.interface_input));
                scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END)
            }
            Message::SetMonitor => {
                self.console_output.push_str(&format!("\n> Set Monitor [{}]", self.monitor_input));
                scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END)
            }
            Message::AddMonitor => {
                if self.interface_input.is_empty() || self.monitor_input.is_empty() {
                    self.console_output.push_str("\n> Error: Interface and Monitor inputs cannot be empty.");
                    return Command::none();
                }
                self.console_output.push_str("\n> Adding monitor...");
                self.is_loading = true; // Indicate loading state
                Command::perform(
                    
                    run_command("iw".to_string(), vec!["dev".to_string(), self.interface_input.clone(),"interface".to_string(),"add".to_string(),self.monitor_input.clone(),"type".to_string(),"monitor".to_string()]), 
                    Message::CommandCompleted // Message to send on completion
                )
            }
            Message::DownInterface => {
                self.console_output.push_str("\n> Downing interface...");
                self.is_loading = true; // Indicate loading state
                Command::perform(
                    run_command("ip".to_string(), vec!["link".to_string(),"set".to_string(), self.down_interface_input.clone(), "down".to_string()]), // Command to run
                    Message::CommandCompleted // Message to send on completion
                )
            }
            Message::UpInterface => {
                self.console_output.push_str("\n> Upping interface...");
                self.is_loading = true;
                Command::perform(
                    run_command(
                        "ip".to_string(),
                        vec![
                            "link".to_string(),
                            "set".to_string(),
                            self.up_interface_input.clone(),
                            "up".to_string(),
                        ],
                    ),
                    Message::CommandCompleted,
                )
            }
            Message::KillNetworkServices => {
                self.console_output.push_str("\n> KIlling netwok services...");
                self.is_loading = true; // Indicate loading state
                Command::perform(
                    run_command("airmon-ng".to_string(), vec!["check".to_string(),"kill".to_string()]), // Command to run
                    Message::CommandCompleted // Message to send on completion
                )
                
            }
            Message::LiftNetworkServices => {
                self.console_output.push_str("\n> Restarting network services...");
                self.is_loading = true; // Indicate loading state
                Command::perform(
                    run_command("systemctl".to_string(), vec!["restart".to_string(),"NetworkManager.service".to_string(),"wpa_supplicant.service".to_string()]), // Command to run
                    Message::CommandCompleted // Message to send on completion
                )
            }
            Message::StartCollectingNetworkList => {
                self.console_output.push_str(&format!("\n> sudo airodump-ng {} --output-format csv -w {}", self.monitor_input,self.path_to_network));
                self.is_loading = true;
                Command::perform(
                    run_command("x-terminal-emulator".to_string(), vec![
                        "-e".to_string(), 
                        "bash".to_string(),
                        "-c".to_string(),
                        format!("sudo airodump-ng {} --output-format csv -w {}", self.monitor_input,self.path_to_network)
                    ]),
                    Message::CommandCompleted
                )
            }
            Message::SelectAPFile => {
                self.console_output.push_str("\n> Opening file selection dialog for AP file...");
                let args = vec![
                    "--file-selection".to_string(),
                    "--title=Select Target AP File".to_string(),
                    "--file-filter=*.csv *.txt".to_string(),
                    "--filename=/root/scans/".to_string(),
                ];
                self.is_loading = true;
                Command::perform(
                    run_command("zenity".to_string(), args),
                    |result| {
                        match result {
                            Ok(output) => {
                                let file_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                                if (!file_path.is_empty()) {
                                    Message::SetPathToApFile(file_path)
                                } else {
                                    Message::CommandCompleted(Ok(
                                        std::process::Output {
                                            status: output.status,
                                            stdout: b"No file selected".to_vec(),
                                            stderr: Vec::new(),
                                        }
                                    ))
                                }
                            },
                            Err(e) => Message::CommandCompleted(Err(e)),
                        }
                    }
                )
            }
            Message::ChooseTargetAP => {
                //pub fn new(bssid: String, first_seen: String, last_seen: String, channel: u8, speed: String, privacy: String, cipher: String, authentication: String, power: i32, beacons: u32, iv: u32, lan_ip: String, id_length: u32, essid: String, key: String)
                if self.path_to_csv_network.is_empty() {
                    self.console_output.push_str("\n> No AP file selected. Please select a file first.");
                    return Command::none();
                }
                self.aps = parse_network_list(self.path_to_csv_network.clone());
                //let aps = self.aps.clone();
                for i in 0..self.aps.len() {

                    self.console_output.push_str(&format!("\n> {}: {}", i,self.aps[i].to_string_less()));
                }

                Command::none()
            }
            Message::DeauthTarget => {
                if (self.target_ap.essid.is_empty() || self.station_mac.len() != 17){
                    self.console_output.push_str("\n> Not enough args");
                    return Command::none();
                }


                self.console_output.push_str(&format!("sudo aireplay-ng --deauth 10 -a {} -c {} {}", 
                    self.target_ap.bssid.clone(), 
                    self.station_mac.clone(),
                    self.monitor_input.clone()));
                
                self.is_loading = true;
                Command::perform(
                    run_command("x-terminal-emulator".to_string(), vec![
                        "-e".to_string(), 
                        "bash".to_string(),
                        "-c".to_string(),
                        format!("sudo aireplay-ng --deauth 10 -a {} -c {} {}", 
                            self.target_ap.bssid.clone(), 
                            self.station_mac.clone(),
                            self.monitor_input.clone())
                    ]), 
                    Message::CommandCompleted
                )
            }
            Message::StartCapturing => {
                if self.target_ap.essid.is_empty() {
                    self.console_output.push_str("\n> No target AP selected. Please select an AP first.");
                    return Command::none();
                }

                self.console_output.push_str(&format!("sudo airodump-ng --bssid {} -c {} {} --output-format cap -w {}",
                    self.target_ap.bssid.clone(),
                    self.target_ap.channel.clone(),
                    self.monitor_input.clone(),
                    self.path_to_network.clone() + &self.target_ap.essid.clone()));

                //self.console_output.push_str("\n> Opening terminal to select target AP...");
                self.is_loading = true;
                Command::perform(
                    run_command("x-terminal-emulator".to_string(), vec![
                        "-e".to_string(), 
                        "bash".to_string(),
                        "-c".to_string(),
                        format!("sudo airodump-ng --bssid {} -c {} {} --output-format cap -w \"{}\"",
                            self.target_ap.bssid.clone(),
                            self.target_ap.channel.clone(),
                            self.monitor_input.clone(),
                            self.path_to_network.clone() + &neutrlize(self.target_ap.essid.clone()))
                    ]), 
                    Message::CommandCompleted
                )
            }
            // --- Handle command completion (Keep or modify as needed) ---
            Message::CommandCompleted(result) => {
                let output_text = match result {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        format!("Status: {}\n--- stdout ---\n{}\n--- stderr ---\n{}", output.status, stdout, stderr)
                    }
                    Err(e) => {
                        format!("Execution failed: {}", e)
                    }
                };
                self.console_output.push_str(&output_text);
                self.console_output.push('\n');
                self.is_loading = false;
                scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END)
            }
            Message::SetPathToApFile(path) => {

                self.path_to_csv_network = path.clone();
                self.console_output.push_str("\n> Path to AP file set to ");
                self.console_output.push_str(&path);
                return scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END);
            }
            Message::ActuallySelect => {
                self.console_output.push_str("\n> Selected");
                if self.selected_n == usize::max_value() || self.selected_n > self.aps.len() as usize {
                    self.console_output.push_str("\n> Invalid selection. Please select a valid AP.");
                    return scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END);

                }
                self.target_ap = self.aps[self.selected_n.clone()].clone();
                self.console_output.push_str(&format!("\n> Selected AP: {}", self.target_ap.essid));
                return scrollable::snap_to(self.scrollable_id.clone(), scrollable::RelativeOffset::END);
            }
        }
    }

    // Define the user interface
    fn view(&self) -> Element<Message> {
        let button_spacing = 10;
        let input_padding = 5;

        // --- Left Controls Column ---
        let controls = column![
            button(text("List Interfaces"))
                .on_press(Message::ListInterfaces),

            row![
                button(text("Set Interface"))
                    .on_press(Message::SetInterface),
                text_input("interface", &self.interface_input)
                    .on_input(Message::InterfaceInputChanged)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

            row![
                button(text("Set Monitor"))
                    .on_press(Message::SetMonitor),
                text_input("monitor", &self.monitor_input)
                    .on_input(Message::MonitorInputChanged)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

            row![
                button(text("Add Monitor"))
                    .on_press(Message::AddMonitor),
                text_input("monitor name", &self.new_monitor_input)
                    .on_input(Message::NewMonitorInputChanged)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

            row![
                button(text("Down"))
                    .on_press(Message::DownInterface),
                text_input("interface name", &self.down_interface_input)
                    .on_input(Message::DownInterfaceInputChanged)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

            row![
                button(text("Up"))
                    .on_press(Message::UpInterface),
                text_input("interface name", &self.up_interface_input)
                    .on_input(Message::UpInterfaceInputChanged)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

            button(text("Kill Network Services"))
                .on_press(Message::KillNetworkServices),

            button(text("Lift Network Services"))
                .on_press(Message::LiftNetworkServices),

            button(text("Start Collecting Network List"))
                .on_press(Message::StartCollectingNetworkList),

            row![
                button(text("Select AP File"))
                    .on_press(Message::SelectAPFile),
                button(text("Print all APs from File"))
                    .on_press(Message::ChooseTargetAP),
                text(&self.target_ap.essid) // Display only
            ].spacing(5).align_items(Alignment::Center),

            row![
                button(text("Select ip from file"))
                    .on_press(Message::ActuallySelect),
                text_input("number of AP", &self.selected_str)
                    .on_input(Message::ActuallySelected)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

            row![
                text_input("Station MAC", &self.station_mac)
                    .on_input(Message::StationMacInputChanged)
                    .padding(input_padding)
            ].spacing(5).align_items(Alignment::Center),

             button(text(format!("Deauth Target [AP: {}, Sta: {}]", self.target_ap.essid, self.station_mac)))
                .on_press(Message::DeauthTarget), // Placeholder action

            button(text(format!("Start Capturing [Selected: {}]", self.target_ap.essid)))
                .on_press(Message::StartCapturing), // Placeholder action

        ]
        .spacing(button_spacing)
        .padding(15)
        .width(Length::FillPortion(5)); // Occupies 40% of width

        // --- Right Console View ---
        let console_view = scrollable(
                text(&self.console_output)
            )
            .id(self.scrollable_id.clone())
            .height(Length::Fill)
            .width(Length::FillPortion(5)); // Occupies 60% of width

        // --- Main Layout (Row) ---
        let content = row![
            controls,
            console_view
        ]
        .spacing(10) // Space between controls and console
        .align_items(Alignment::Start) // Align items to the top
        .width(Length::Fill)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x() // Center the row horizontally if window is wider
            .center_y() // Center the row vertically if window is taller
            .into()
    }
}

// Main function to run the application
fn main() -> iced::Result {

    // --- Check for root privileges ---
    if !geteuid().is_root() {
        eprintln!("Error: This application requires root privileges to manage network interfaces.");
        eprintln!("Please run it using 'sudo'.");
        exit(1); // Exit if not root
    }
    // --- End check ---



    ConsoleApp::run(Settings {
        window: iced::window::Settings {
            size: (Size::new(1000.0, 800.0)), // Initial window size
            min_size: Some(Size::new(1000.0, 800.0)),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}