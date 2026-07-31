use super::calllib::AP;
use iced::widget::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobKind {
    Scan,
    Capture,
    Deauth,
    Crack,
}

impl JobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            JobKind::Scan => "Scan",
            JobKind::Capture => "Capture",
            JobKind::Deauth => "Deauth",
            JobKind::Crack => "Crack",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: u64,
    pub kind: JobKind,
    pub label: String,
    pub summary: String,
    pub pid: u32,
    pub pgid: i32,
    pub running: bool,
}

pub struct ConsoleApp {
    pub interfaces: Vec<String>,
    pub monitor_interfaces: Vec<String>,
    pub selected_interface: Option<String>,
    pub selected_monitor: Option<String>,
    pub selected_str: String,
    pub path_to_network: String,
    pub console_output: String,
    pub scrollable_id: Id,
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
    pub show_settings: bool,
    pub storage_location: String,
    pub storage_location_input: String,
    pub remote_server_credentials: String,
    pub remote_server_credentials_input: String,
    pub local_password_list: String,
    pub local_password_list_input: String,
    pub terminal: String,
    pub terminal_input: String,
    pub cap_file_path: String,
    pub show_console: bool,
    pub sort_column: usize,
    pub sort_descending: bool,
    pub filter_text: String,
    /// Sudo password entered at startup (memory only, never written to disk)
    pub sudo_password: String,
    pub jobs: Vec<Job>,
    pub selected_job_id: Option<u64>,
    pub next_job_id: u64,
}
