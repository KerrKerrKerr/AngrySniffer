use super::calllib::AP;
use iced::widget::scrollable;

// Define the application state
pub struct ConsoleApp {
    pub interfaces: Vec<String>,
    pub monitor_interfaces: Vec<String>,
    pub selected_interface: Option<String>,
    pub selected_monitor: Option<String>,
    pub selected_str: String,
    pub path_to_network: String,
    pub console_output: String,
    pub scrollable_id: scrollable::Id,
    pub is_loading: bool,
    pub new_monitor_input: String,
    pub down_interface_input: String,
    pub up_interface_input: String,
    pub target_ap: AP,
    pub aps: Vec<AP>,
    pub path_to_csv_network: String,
    pub selected_n: usize,
    pub station_mac: String,
    pub network_services_killed: bool,
    pub show_exit_dialog: bool,
}
