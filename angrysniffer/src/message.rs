use std::process::Output;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    // --- Input changes ---
    NewMonitorInputChanged(String),
    ActuallySelected(String),
    StationMacInputChanged(String),
    // --- Button presses ---
    RefreshInterfaces,
    RefreshMonitorInterfaces,
    ActuallySelect,
    ChooseTargetAP,
    ListInterfaces,
    SetInterface,
    SetMonitor,
    AddMonitor,
    DownInterface,
    UpInterface,
    KillNetworkServices,
    LiftNetworkServices,
    StartCollectingNetworkList,
    SelectAPFile,
    DeauthTarget,
    StartCapturing,
    SetPathToApFile(String),
    InterfaceSelected(Option<String>),
    MonitorSelected(Option<String>),
    DownInterfaceSelected(Option<String>),
    UpInterfaceSelected(Option<String>),
    // --- Window events ---

    // --- Existing ---
    CommandCompleted(Result<Output, Arc<std::io::Error>>),
}
