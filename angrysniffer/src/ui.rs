use crate::message::Message;
use crate::state::ConsoleApp;
use crate::calllib::AP;
use iced::{
    widget::{
        button, column, container, pick_list, row, text, text_input,
        scrollable::Scrollable, space, rule,
    },
    Alignment, Color, Element, Length,
    Border, Shadow,
    Theme,
};
use iced::widget::button::Status;

const BG_DARK: Color = Color::from_rgb(0.08, 0.08, 0.10);
const BG_INPUT: Color = Color::from_rgb(0.16, 0.16, 0.20);
const TEXT_PRIMARY: Color = Color::from_rgb(0.85, 0.85, 0.90);
const TEXT_MUTED: Color = Color::from_rgb(0.55, 0.55, 0.60);
const ACCENT_BLUE: Color = Color::from_rgb(0.20, 0.45, 0.85);
const ACCENT_GREEN: Color = Color::from_rgb(0.15, 0.60, 0.35);
const ACCENT_RED: Color = Color::from_rgb(0.75, 0.20, 0.20);
const ACCENT_ORANGE: Color = Color::from_rgb(0.85, 0.55, 0.10);
const BORDER_COLOR: Color = Color::from_rgb(0.25, 0.25, 0.30);
const TABLE_HEADER_BG: Color = Color::from_rgb(0.15, 0.15, 0.20);
const TABLE_ROW_ALT: Color = Color::from_rgb(0.10, 0.10, 0.13);
const TABLE_ROW_SELECTED: Color = Color::from_rgb(0.18, 0.30, 0.50);
const CONSOLE_BG: Color = Color::from_rgb(0.05, 0.05, 0.07);

fn btn_style(base: Color, hover: Color) -> impl Fn(&Theme, Status) -> button::Style {
    move |_theme: &Theme, status: Status| -> button::Style {
        let mut s = button::Style {
            background: Some(iced::Background::Color(base)),
            text_color: TEXT_PRIMARY,
            border: Border {
                color: base,
                width: 1.0,
                radius: 6.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        };
        match status {
            Status::Hovered => {
                s.background = Some(iced::Background::Color(hover));
                s.shadow = Shadow {
                    color: Color { a: 0.3, ..base },
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                };
                s
            }
            Status::Disabled => {
                s.background = Some(iced::Background::Color(Color::from_rgb(0.15, 0.15, 0.18)));
                s.text_color = TEXT_MUTED;
                s
            }
            _ => s,
        }
    }
}

fn input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(BG_INPUT),
        border: Border {
            color: match status {
                text_input::Status::Focused { .. } => ACCENT_BLUE,
                _ => BORDER_COLOR,
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: TEXT_MUTED,
        placeholder: TEXT_MUTED,
        value: TEXT_PRIMARY,
        selection: ACCENT_BLUE,
    }
}

fn dark_bg(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_DARK)),
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

fn header_rounded(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(TABLE_HEADER_BG)),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: 6.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn picklist_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let border_color = match status {
        pick_list::Status::Hovered => ACCENT_BLUE,
        pick_list::Status::Opened { .. } => BORDER_COLOR,
        pick_list::Status::Active => BORDER_COLOR,
    };
    pick_list::Style {
        background: BG_INPUT.into(),
        text_color: TEXT_PRIMARY,
        placeholder_color: TEXT_MUTED,
        handle_color: TEXT_MUTED,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: 6.0.into(),
        },
    }
}

fn section_title(label: &str) -> Element<'_, Message> {
    text(label).color(ACCENT_BLUE).size(11).into()
}

fn action_btn(label: String, color: Color, hover: Color, msg: Message) -> Element<'static, Message> {
    button(text(label).size(11))
        .style(btn_style(color, hover))
        .on_press(msg)
        .padding(6)
        .width(Length::Fill)
        .into()
}

fn small_btn(label: &str, color: Color, hover: Color, msg: Message) -> Element<'static, Message> {
    button(text(String::from(label)).size(11))
        .style(btn_style(color, hover))
        .on_press(msg)
        .padding(6)
        .width(Length::Fill)
        .into()
}

fn divider() -> Element<'static, Message> {
    rule::horizontal(1)
        .style(move |_: &Theme| rule::Style {
            color: BORDER_COLOR,
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: false,
        })
        .into()
}



pub fn view(app: &ConsoleApp) -> Element<'_, Message> {
    if app.show_settings {
        return settings_view(app);
    }

    let input_padding = 4;

    let list_btn = action_btn(
        "List Interfaces".to_string(),
        ACCENT_BLUE, Color::from_rgb(0.30, 0.55, 0.95),
        Message::ListInterfaces,
    );

    let interface_pick = pick_list(
        app.interfaces.clone(),
        app.selected_interface.clone(),
        |s: String| Message::InterfaceSelected(Some(s)),
    )
    .placeholder("Select interface")
    .width(Length::Fill)
    .style(picklist_style);

    let monitor_pick = pick_list(
        app.monitor_interfaces.clone(),
        app.selected_monitor.clone(),
        |s: String| Message::MonitorSelected(Some(s)),
    )
    .placeholder("Select monitor interface")
    .width(Length::Fill)
    .style(picklist_style);

    let add_mon_btn = small_btn("Add Monitor", ACCENT_GREEN, Color::from_rgb(0.25, 0.70, 0.45), Message::AddMonitor);

    let mon_input = text_input("monitor name", &app.new_monitor_input)
        .on_input(Message::NewMonitorInputChanged)
        .padding(input_padding)
        .style(input_style)
        .width(Length::Fill);

    let down_btn = small_btn("Down", ACCENT_ORANGE, Color::from_rgb(0.95, 0.65, 0.20), Message::DownInterface);

    let down_pick = pick_list(
        app.interfaces.clone(),
        Some(app.down_interface_input.clone()).filter(|s| !s.is_empty()),
        |s: String| Message::DownInterfaceSelected(Some(s)),
    )
    .placeholder("iface")
    .width(Length::Fill)
    .style(picklist_style);

    let up_btn = small_btn("Up", ACCENT_GREEN, Color::from_rgb(0.25, 0.70, 0.45), Message::UpInterface);

    let up_pick = pick_list(
        app.interfaces.clone(),
        Some(app.up_interface_input.clone()).filter(|s| !s.is_empty()),
        |s: String| Message::UpInterfaceSelected(Some(s)),
    )
    .placeholder("iface")
    .width(Length::Fill)
    .style(picklist_style);

    let kill_btn = action_btn(
        "Kill Network Services".to_string(),
        ACCENT_RED, Color::from_rgb(0.85, 0.30, 0.30),
        Message::KillNetworkServices,
    );

    let lift_btn = action_btn(
        "Lift Network Services".to_string(),
        ACCENT_BLUE, Color::from_rgb(0.30, 0.55, 0.95),
        Message::LiftNetworkServices,
    );

    let collect_btn = action_btn(
        "Start Collecting Network List".to_string(),
        ACCENT_BLUE, Color::from_rgb(0.30, 0.55, 0.95),
        Message::StartCollectingNetworkList,
    );

    let select_ap_btn = action_btn(
        "Select AP File".to_string(),
        ACCENT_BLUE, Color::from_rgb(0.30, 0.55, 0.95),
        Message::SelectAPFile,
    );

    let station_input = text_input(
        "Station MAC",
        &app.station_mac,
    )
    .on_input(Message::StationMacInputChanged)
    .padding(input_padding)
    .style(input_style)
    .width(Length::Fill);

    let target_label = if app.target_ap.essid.is_empty() {
        "none".to_string()
    } else {
        app.target_ap.essid.clone()
    };
    let station_label = if app.station_mac.is_empty() {
        "none".to_string()
    } else {
        app.station_mac.clone()
    };

    let deauth_btn = action_btn(
        format!("Deauth [AP: {}, Sta: {}]", target_label, station_label),
        ACCENT_RED, Color::from_rgb(0.85, 0.30, 0.30),
        Message::DeauthTarget,
    );

    let cap_label = if app.target_ap.essid.is_empty() {
        "no target".to_string()
    } else {
        app.target_ap.essid.clone()
    };

    let capture_btn = action_btn(
        format!("Start Capturing [{}]", cap_label),
        ACCENT_GREEN, Color::from_rgb(0.25, 0.70, 0.45),
        Message::StartCapturing,
    );

    let crack_btn = action_btn(
        "Crack Captured Handshake".to_string(),
        ACCENT_ORANGE, Color::from_rgb(0.95, 0.65, 0.20),
        Message::CrackCapturedHandshake,
    );

    let crack_local_btn = action_btn(
        "Crack Capture File Locally".to_string(),
        ACCENT_ORANGE, Color::from_rgb(0.95, 0.65, 0.20),
        Message::CrackCaptureFileLocally,
    );

    let console_toggle = if app.show_console {
        small_btn("Hide Console", Color::from_rgb(0.25, 0.25, 0.30), Color::from_rgb(0.35, 0.35, 0.40), Message::ToggleConsole)
    } else {
        small_btn("Show Console", Color::from_rgb(0.25, 0.25, 0.30), Color::from_rgb(0.35, 0.35, 0.40), Message::ToggleConsole)
    };

    let controls = column![
        small_btn("Settings", Color::from_rgb(0.25, 0.25, 0.30), Color::from_rgb(0.35, 0.35, 0.40), Message::OpenSettings),
        divider(),
        section_title("Interface"),
        list_btn,
        interface_pick,
        monitor_pick,
        row![add_mon_btn, mon_input].spacing(4).align_y(Alignment::Center).width(Length::Fill),
        row![down_pick, down_btn].spacing(4).align_y(Alignment::Center).width(Length::Fill),
        row![up_pick, up_btn].spacing(4).align_y(Alignment::Center).width(Length::Fill),
        row![kill_btn, lift_btn].spacing(4).width(Length::Fill),
        collect_btn,
        divider(),
        section_title("Target"),
        select_ap_btn,
        station_input,
        divider(),
        section_title("Attack"),
        deauth_btn,
        capture_btn,
        divider(),
        section_title("Cracking"),
        row![crack_btn, crack_local_btn].spacing(4).width(Length::Fill),
        space::vertical().height(Length::Fill),
        console_toggle,
    ]
    .spacing(5)
    .padding(10)
    .width(Length::Fill);

    let sidebar = container(controls)
        .width(Length::Fill);

    let mut right_column: Vec<Element<'_, Message>> = Vec::new();

    right_column.push(ap_table_view(app));

    if app.show_console {
        let console_scroll = Scrollable::new(
            text(&app.console_output)
                .color(Color::from_rgb(0.30, 0.85, 0.40))
                .size(11),
        )
        .id(app.scrollable_id.clone())
        .height(Length::Fill);

        let console_view = container(console_scroll)
            .style(|_: &Theme| container::Style {
                text_color: None,
                background: Some(iced::Background::Color(CONSOLE_BG)),
                border: Border {
                    color: BORDER_COLOR,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Shadow::default(),
                snap: false,
            })
            .padding(8)
            .height(Length::Fill);

        right_column.push(console_view.into());
    }

    let right_panel = column(right_column)
        .spacing(8)
        .height(Length::Fill)
        .width(Length::Fill);

    let separator = container(space::vertical())
        .width(1)
        .style(|_: &Theme| container::Style {
            text_color: None,
            background: Some(iced::Background::Color(BORDER_COLOR)),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        });

    let content = row![
        sidebar.width(Length::FillPortion(2)),
        separator,
        right_panel.width(Length::FillPortion(7)),
    ]
    .spacing(0)
    .align_y(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill);

    container(content)
        .style(dark_bg)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(0)
        .into()
}

fn settings_view(app: &ConsoleApp) -> Element<'_, Message> {
    let settings_content = column![
        text("Settings").color(ACCENT_BLUE).size(16),
        space::vertical().height(Length::FillPortion(1)),
        settings_field_row(
            "Storage location:",
            &app.storage_location_input,
            Message::StorageLocationInputChanged,
            Message::OpenStorageLocationDialog,
        ),
        settings_field_row(
            "Remote credentials:",
            &app.remote_server_credentials_input,
            Message::RemoteServerCredentialsInputChanged,
            Message::OpenRemoteServerCredentialsDialog,
        ),
        settings_field_row(
            "Password list:",
            &app.local_password_list_input,
            Message::LocalPasswordListInputChanged,
            Message::OpenLocalPasswordListDialog,
        ),
        space::vertical().height(Length::FillPortion(1)),
        row![
            button(text("Save").size(12))
                .style(btn_style(ACCENT_GREEN, Color::from_rgb(0.25, 0.70, 0.45)))
                .on_press(Message::SaveSettings)
                .padding(10)
                .width(Length::Fill),
            button(text("Back").size(12))
                .style(btn_style(
                    Color::from_rgb(0.25, 0.25, 0.30),
                    Color::from_rgb(0.35, 0.35, 0.40),
                ))
                .on_press(Message::CloseSettings)
                .padding(10)
                .width(Length::Fill),
        ]
        .spacing(10)
        .width(Length::Fill),
    ]
    .spacing(15)
    .padding(25)
    .max_width(600)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Alignment::Center);

    let panel = container(settings_content)
        .width(Length::Fill)
        .height(Length::Shrink);

    container(panel)
        .style(dark_bg)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn settings_field_row<'a>(
    label: &'a str,
    value: &'a str,
    on_input: fn(String) -> Message,
    on_browse: Message,
) -> Element<'a, Message> {
    row![
        text(label).color(TEXT_PRIMARY).size(13).width(Length::FillPortion(3)),
        text_input("Path", value)
            .on_input(on_input)
            .padding(6)
            .style(input_style)
            .width(Length::FillPortion(5)),
        button(text("Browse").size(12))
            .style(btn_style(
                Color::from_rgb(0.25, 0.25, 0.30),
                Color::from_rgb(0.35, 0.35, 0.40),
            ))
            .on_press(on_browse)
            .padding(6),
    ]
    .spacing(10)
    .align_y(Alignment::Center)
    .into()
}

fn col_header(label: &str, col: usize, sort_col: usize, desc: bool, flex: u16) -> Element<'static, Message> {
    let arrow = if col == sort_col {
        if desc { " ▼" } else { " ▲" }
    } else {
        ""
    };
    button(text(format!("{}{}", label, arrow)).size(11))
        .on_press(Message::SortByColumn(col))
        .style(move |_: &Theme, status: Status| -> button::Style {
            let bg = match status {
                Status::Hovered => Color::from_rgb(0.22, 0.35, 0.55),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: if col == sort_col { ACCENT_BLUE } else { TEXT_PRIMARY },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                snap: false,
            }
        })
        .padding(0)
        .width(Length::FillPortion(flex))
        .into()
}

fn ap_table_view(app: &ConsoleApp) -> Element<'_, Message> {
    let filter_input = text_input("Filter by ESSID...", &app.filter_text)
        .on_input(Message::FilterTextChanged)
        .padding(4)
        .style(input_style)
        .width(Length::Fill);

    let header = container(
        row![
            col_header("#", 0, app.sort_column, app.sort_descending, 1),
            col_header("BSSID", 1, app.sort_column, app.sort_descending, 4),
            col_header("Power", 2, app.sort_column, app.sort_descending, 2),
            col_header("Ch", 3, app.sort_column, app.sort_descending, 1),
            col_header("ESSID", 4, app.sort_column, app.sort_descending, 4),
            col_header("Privacy", 5, app.sort_column, app.sort_descending, 2),
        ]
        .spacing(4)
        .padding(6)
        .align_y(Alignment::Center),
    )
    .style(header_rounded)
    .width(Length::Fill);

    let filtered: Vec<&AP> = if app.filter_text.is_empty() {
        app.aps.iter().collect()
    } else {
        let lower = app.filter_text.to_lowercase();
        app.aps.iter().filter(|ap| ap.essid.to_lowercase().contains(&lower)).collect()
    };

    let display_aps = if app.sort_column > 0 || app.sort_descending {
        let mut cloned: Vec<AP> = filtered.iter().map(|ap| (*ap).clone()).collect();
        let cmp = |a: &AP, b: &AP| -> std::cmp::Ordering {
            match app.sort_column {
                0 => a.essid.cmp(&b.essid),
                1 => a.bssid.cmp(&b.bssid),
                2 => a.power.cmp(&b.power),
                3 => a.channel.cmp(&b.channel),
                4 => a.essid.cmp(&b.essid),
                5 => a.privacy.cmp(&b.privacy),
                _ => std::cmp::Ordering::Equal,
            }
        };
        if app.sort_descending {
            cloned.sort_by(|a, b| cmp(b, a));
        } else {
            cloned.sort_by(cmp);
        }
        cloned
    } else {
        filtered.iter().map(|ap| (*ap).clone()).collect()
    };

    let mut rows: Vec<Element<'_, Message>> = Vec::new();

    for (i, ap) in display_aps.iter().enumerate() {
        let bssid = ap.bssid.clone();
        let power = format!("{} dBm", ap.power);
        let channel = format!("Ch {}", ap.channel);
        let essid = if ap.essid.is_empty() { "<hidden>".to_string() } else { ap.essid.clone() };
        let privacy = ap.privacy.clone();
        let is_selected = app.selected_n < app.aps.len()
            && ap.bssid == app.aps[app.selected_n].bssid
            && ap.essid == app.aps[app.selected_n].essid;

        let power_color = if ap.power > -50 {
            Color::from_rgb(0.20, 0.85, 0.30)
        } else if ap.power > -70 {
            Color::from_rgb(0.85, 0.85, 0.20)
        } else {
            Color::from_rgb(0.85, 0.30, 0.20)
        };

        let privacy_color = match privacy.as_str() {
            "WPA2" => ACCENT_GREEN,
            "WPA" => ACCENT_ORANGE,
            "WEP" => ACCENT_RED,
            _ => TEXT_MUTED,
        };

        let essid_color = if essid == "<hidden>" { TEXT_MUTED } else { TEXT_PRIMARY };
        let idx_color = if is_selected { TEXT_PRIMARY } else { TEXT_MUTED };

        let actual_idx = app.aps.iter().position(|a| a.bssid == ap.bssid && a.essid == ap.essid)
            .unwrap_or(usize::max_value());

        let row_entry = button(
            row![
                text(format!("{}", i)).color(idx_color).size(11).width(Length::FillPortion(1)),
                text(bssid).color(TEXT_PRIMARY).size(11).width(Length::FillPortion(4)),
                text(power).color(power_color).size(11).width(Length::FillPortion(2)),
                text(channel).color(TEXT_PRIMARY).size(11).width(Length::FillPortion(1)),
                text(essid).color(essid_color).size(11).width(Length::FillPortion(4)),
                text(privacy).color(privacy_color).size(11).width(Length::FillPortion(2)),
            ]
            .spacing(4)
            .padding(iced::Padding::from([3, 6]))
            .align_y(Alignment::Center),
        )
        .on_press(Message::SelectApFromTable(actual_idx))
        .style(move |_: &Theme, status: Status| -> button::Style {
            let bg = match status {
                Status::Hovered => Color::from_rgb(0.22, 0.35, 0.55),
                _ if is_selected => TABLE_ROW_SELECTED,
                _ => {
                    if i % 2 == 0 { TABLE_ROW_ALT } else { BG_DARK }
                }
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: TEXT_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                snap: false,
            }
        })
        .padding(0)
        .width(Length::Fill);

        rows.push(row_entry.into());
    }

    let summary: Element<'_, Message> = if !display_aps.is_empty() && app.selected_n < app.aps.len() {
        text(format!(
            "{} APs (filtered: {}) - Selected: {}",
            app.aps.len(),
            if app.filter_text.is_empty() { "all".to_string() } else { display_aps.len().to_string() },
            app.aps[app.selected_n].essid
        ))
        .color(TEXT_MUTED)
        .size(10)
        .into()
    } else if !app.aps.is_empty() {
        text(format!(
            "{} APs (filtered: {})",
            app.aps.len(),
            display_aps.len()
        ))
        .color(TEXT_MUTED)
        .size(10)
        .into()
    } else {
        text("Select a CSV file to load APs")
            .color(TEXT_MUTED)
            .size(11)
            .into()
    };

    let mut table_content: Vec<Element<'_, Message>> = Vec::with_capacity(rows.len() + 1);
    table_content.push(header.into());
    table_content.extend(rows);

    let table_scroll = Scrollable::new(column(table_content).spacing(1))
        .height(Length::Fill);

    let table = column![
        text("Access Points").color(ACCENT_BLUE).size(12).width(Length::Fill),
        filter_input,
        table_scroll,
        summary,
    ]
    .spacing(4)
    .height(Length::FillPortion(1));

    container(table)
        .padding(8)
        .height(Length::FillPortion(1))
        .width(Length::Fill)
        .into()
}