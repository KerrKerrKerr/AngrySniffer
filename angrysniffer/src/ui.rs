use crate::message::Message;
use crate::state::ConsoleApp;
use iced::{
    widget::{button, column, container, pick_list, row, scrollable, text, text_input},
    Alignment, Element, Length, Theme,
};

pub fn view(app: &ConsoleApp) -> Element<Message> {
    let button_spacing = 10;
    let input_padding = 5;

    if app.show_settings {
        let settings_controls = column![
            text("Settings Window"),
            row![
                text("Default storage location:"),
                text_input("Path", &app.storage_location_input)
                    .on_input(Message::StorageLocationInputChanged)
                    .padding(5),
                button(text("Browse")).on_press(Message::OpenStorageLocationDialog),
            ]
            .spacing(10),
            row![
                text("Remote server credentials:"),
                text_input("Path", &app.remote_server_credentials_input)
                    .on_input(Message::RemoteServerCredentialsInputChanged)
                    .padding(5),
                button(text("Browse")).on_press(Message::OpenRemoteServerCredentialsDialog),
            ]
            .spacing(10),
            row![
                text("Local password list:"),
                text_input("Path", &app.local_password_list_input)
                    .on_input(Message::LocalPasswordListInputChanged)
                    .padding(5),
                button(text("Browse")).on_press(Message::OpenLocalPasswordListDialog),
                ]
                .spacing(10),
            button(text("Save")).on_press(Message::SaveSettings),
            button(text("Close Settings")).on_press(Message::CloseSettings),
        ]
        .spacing(10)
        .padding(20)
        .width(Length::FillPortion(5));

        return container(settings_controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();
    }

    let controls = column![
        button(text("Settings")).on_press(Message::OpenSettings),
        button(text("List Interfaces")).on_press(Message::ListInterfaces),
        row![
            pick_list(
                app.interfaces.clone(),
                app.selected_interface.clone(),
                |s: String| Message::InterfaceSelected(Some(s)),
            )
            .placeholder("Select interface")
            .on_open(Message::RefreshInterfaces)
            .width(Length::FillPortion(2))
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            pick_list(
                app.monitor_interfaces.clone(),
                app.selected_monitor.clone(),
                |s: String| Message::MonitorSelected(Some(s)),
            )
            .placeholder("Select monitor interface")
            .on_open(Message::RefreshMonitorInterfaces)
            .width(Length::FillPortion(2))
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            button(text("Add Monitor")).on_press(Message::AddMonitor),
            text_input("monitor name", &app.new_monitor_input)
                .on_input(Message::NewMonitorInputChanged)
                .padding(input_padding)
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            button(text("Down")).on_press(Message::DownInterface),
            pick_list(
                app.interfaces.clone(),
                Some(app.down_interface_input.clone()).filter(|s| !s.is_empty()),
                |s: String| Message::DownInterfaceSelected(Some(s)),
            )
            .placeholder("Select interface to down")
            .on_open(Message::RefreshInterfaces)
            .width(Length::FillPortion(2))
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            button(text("Up")).on_press(Message::UpInterface),
            pick_list(
                app.interfaces.clone(),
                Some(app.up_interface_input.clone()).filter(|s| !s.is_empty()),
                |s: String| Message::UpInterfaceSelected(Some(s)),
            )
            .placeholder("Select interface to up")
            .on_open(Message::RefreshInterfaces)
            .width(Length::FillPortion(2))
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        button(text("Kill Network Services")).on_press(Message::KillNetworkServices),
        button(text("Lift Network Services")).on_press(Message::LiftNetworkServices),
        button(text("Start Collecting Network List"))
            .on_press(Message::StartCollectingNetworkList),
        row![
            button(text("Select AP File")).on_press(Message::SelectAPFile),
            button(text("Print all APs from File")).on_press(Message::ChooseTargetAP),
            text(&app.target_ap.essid)
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            button(text("Select ip from file")).on_press(Message::ActuallySelect),
            text_input("number of AP", &app.selected_str)
                .on_input(Message::ActuallySelected)
                .padding(input_padding)
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        row![
            text_input("Station MAC", &app.station_mac)
                .on_input(Message::StationMacInputChanged)
                .padding(input_padding)
        ]
        .spacing(5)
        .align_items(Alignment::Center),
        button(text(format!(
            "Deauth Target [AP: {}, Sta: {}]",
            app.target_ap.essid, app.station_mac
        )))
        .on_press(Message::DeauthTarget),
        button(text(format!(
            "Start Capturing [Selected: {}]",
            app.target_ap.essid
        )))
        .on_press(Message::StartCapturing),
        button(text("Crack Captured Handshake"))
            .on_press(Message::CrackCapturedHandshake),
        button(text("Crack capture file locally"))
            .on_press(Message::CrackCaptureFileLocally),
    ]
    .spacing(button_spacing)
    .padding(15)
    .width(Length::FillPortion(5));

    let console_view = scrollable(text(&app.console_output))
        .id(app.scrollable_id.clone())
        .height(Length::Fill)
        .width(Length::FillPortion(5));

    let content = row![controls, console_view]
        .spacing(10)
        .align_items(Alignment::Start)
        .width(Length::Fill)
        .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
