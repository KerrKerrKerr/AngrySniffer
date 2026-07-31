use crate::calllib::AP;
use crate::message::Message;
use crate::state::ConsoleApp;
use iced::widget::button::Status;
use iced::widget::{
    button, column, container, mouse_area, pick_list, row, rule, scrollable::Scrollable, space,
    text, text_input,
};
use iced::{Alignment, Border, Color, Element, Length, Shadow, Theme};

// ─── Palette (flat, muted) ──────────────────────────────────────────────────

const BG_APP: Color = Color::from_rgb(0.09, 0.09, 0.10);
const BG_PANEL: Color = Color::from_rgb(0.11, 0.11, 0.12);
const BG_HEADER: Color = Color::from_rgb(0.13, 0.13, 0.14);
const BG_INPUT: Color = Color::from_rgb(0.14, 0.14, 0.15);
const BG_BTN: Color = Color::from_rgb(0.18, 0.18, 0.20);
const BG_BTN_HOVER: Color = Color::from_rgb(0.24, 0.24, 0.27);
const BG_OVERLAY: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.55,
};
const BG_CONSOLE: Color = Color::from_rgb(0.06, 0.06, 0.07);

const TEXT_PRIMARY: Color = Color::from_rgb(0.82, 0.82, 0.84);
const TEXT_MUTED: Color = Color::from_rgb(0.50, 0.50, 0.53);
const TEXT_DIM: Color = Color::from_rgb(0.38, 0.38, 0.40);

const BORDER: Color = Color::from_rgb(0.22, 0.22, 0.24);
const BORDER_SUBTLE: Color = Color::from_rgb(0.17, 0.17, 0.19);

const ACCENT: Color = Color::from_rgb(0.45, 0.55, 0.70);
const DANGER_FG: Color = Color::from_rgb(0.78, 0.48, 0.48);
const SUCCESS_FG: Color = Color::from_rgb(0.48, 0.68, 0.52);
const WARNING_FG: Color = Color::from_rgb(0.78, 0.68, 0.42);

const TABLE_ROW_ALT: Color = Color::from_rgb(0.10, 0.10, 0.11);
const TABLE_SELECTED: Color = Color::from_rgb(0.16, 0.18, 0.22);
const TABLE_HOVER: Color = Color::from_rgb(0.15, 0.16, 0.18);

const POWER_STRONG: Color = Color::from_rgb(0.45, 0.70, 0.50);
const POWER_MID: Color = Color::from_rgb(0.72, 0.70, 0.42);
const POWER_WEAK: Color = Color::from_rgb(0.72, 0.48, 0.45);
const CONSOLE_FG: Color = Color::from_rgb(0.45, 0.70, 0.50);

const R: f32 = 2.0;

// ─── Styles ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum BtnKind {
    Default,
    Danger,
    Ghost,
}

fn btn_style(kind: BtnKind) -> impl Fn(&Theme, Status) -> button::Style {
    move |_theme: &Theme, status: Status| -> button::Style {
        let text_color = match kind {
            BtnKind::Danger => DANGER_FG,
            _ => TEXT_PRIMARY,
        };
        let (bg, border_c) = match (kind, status) {
            (BtnKind::Ghost, Status::Hovered) => (BG_BTN, BORDER),
            (BtnKind::Ghost, _) => (Color::TRANSPARENT, BORDER),
            (_, Status::Hovered) => (BG_BTN_HOVER, BORDER),
            (_, Status::Disabled) => (BG_PANEL, BORDER_SUBTLE),
            _ => (BG_BTN, BORDER),
        };
        let tc = if matches!(status, Status::Disabled) {
            TEXT_DIM
        } else {
            text_color
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color: tc,
            border: Border {
                color: border_c,
                width: 1.0,
                radius: R.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

fn input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: iced::Background::Color(BG_INPUT),
        border: Border {
            color: match status {
                text_input::Status::Focused { .. } => ACCENT,
                text_input::Status::Hovered => Color::from_rgb(0.30, 0.30, 0.32),
                _ => BORDER,
            },
            width: 1.0,
            radius: R.into(),
        },
        icon: TEXT_MUTED,
        placeholder: TEXT_DIM,
        value: TEXT_PRIMARY,
        selection: Color::from_rgb(0.25, 0.30, 0.40),
    }
}

fn picklist_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let border_color = match status {
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => ACCENT,
        pick_list::Status::Active => BORDER,
    };
    pick_list::Style {
        background: BG_INPUT.into(),
        text_color: TEXT_PRIMARY,
        placeholder_color: TEXT_DIM,
        handle_color: TEXT_MUTED,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: R.into(),
        },
    }
}

fn app_bg(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(TEXT_PRIMARY),
        background: Some(iced::Background::Color(BG_APP)),
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

fn panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_PANEL)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: R.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn header_bar(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_HEADER)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn console_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_CONSOLE)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: R.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn overlay_bg(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_OVERLAY)),
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

fn settings_panel(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_PANEL)),
        border: Border {
            color: BORDER,
            width: 1.0,
            radius: R.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

fn header_row_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: None,
        background: Some(iced::Background::Color(BG_HEADER)),
        border: Border {
            color: BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

// ─── Primitives ─────────────────────────────────────────────────────────────

fn action_btn(label: impl Into<String>, kind: BtnKind, msg: Message) -> Element<'static, Message> {
    button(text(label.into()).size(12))
        .style(btn_style(kind))
        .on_press(msg)
        .padding([7, 10])
        .width(Length::Fill)
        .into()
}

fn compact_btn(label: impl Into<String>, kind: BtnKind, msg: Message) -> Element<'static, Message> {
    button(text(label.into()).size(11))
        .style(btn_style(kind))
        .on_press(msg)
        .padding([6, 8])
        .width(Length::Fill)
        .into()
}

fn section_label(label: &str) -> Element<'_, Message> {
    column![
        text(label.to_uppercase()).color(TEXT_MUTED).size(10),
        divider(),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

fn divider() -> Element<'static, Message> {
    rule::horizontal(1)
        .style(|_: &Theme| rule::Style {
            color: BORDER_SUBTLE,
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: false,
        })
        .into()
}

fn status_item<'a>(label: &'a str, value: &'a str, value_color: Color) -> Element<'a, Message> {
    row![
        text(format!("{label}:")).color(TEXT_DIM).size(11),
        text(value).color(value_color).size(11),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .into()
}

fn or_none(s: &str) -> &str {
    if s.is_empty() { "—" } else { s }
}

// ─── Root ───────────────────────────────────────────────────────────────────

pub fn view(app: &ConsoleApp) -> Element<'_, Message> {
    if app.show_settings {
        return settings_view(app);
    }

    let mut shell_parts: Vec<Element<'_, Message>> = vec![status_bar(app)];
    if !app.jobs.is_empty() {
        shell_parts.push(jobs_bar(app));
    }
    shell_parts.push(
        row![
            container(sidebar(app))
                .width(Length::FillPortion(2))
                .height(Length::Fill),
            v_sep(),
            container(main_panel(app))
                .width(Length::FillPortion(7))
                .height(Length::Fill),
        ]
        .height(Length::Fill)
        .width(Length::Fill)
        .into(),
    );

    let shell = column(shell_parts)
        .width(Length::Fill)
        .height(Length::Fill);

    container(shell)
        .style(app_bg)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn v_sep() -> Element<'static, Message> {
    container(space::vertical())
        .width(1)
        .height(Length::Fill)
        .style(|_: &Theme| container::Style {
            text_color: None,
            background: Some(iced::Background::Color(BORDER_SUBTLE)),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        })
        .into()
}

// ─── Custom title bar (client-side decorations) ─────────────────────────────

fn win_btn(label: &'static str, kind: WinBtn) -> Element<'static, Message> {
    let msg = match kind {
        WinBtn::Minimize => Message::WindowMinimize,
        WinBtn::Maximize => Message::WindowToggleMaximize,
        WinBtn::Close => Message::WindowClose,
    };
    button(text(label).size(12).center())
        .style(win_btn_style(kind))
        .on_press(msg)
        .padding(0)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(32.0))
        .into()
}

#[derive(Clone, Copy)]
enum WinBtn {
    Minimize,
    Maximize,
    Close,
}

fn win_btn_style(kind: WinBtn) -> impl Fn(&Theme, Status) -> button::Style {
    move |_theme: &Theme, status: Status| -> button::Style {
        let (bg, tc) = match (kind, status) {
            (WinBtn::Close, Status::Hovered) => {
                (Color::from_rgb(0.55, 0.18, 0.18), Color::WHITE)
            }
            (_, Status::Hovered) => (BG_BTN_HOVER, TEXT_PRIMARY),
            _ => (Color::TRANSPARENT, TEXT_MUTED),
        };
        button::Style {
            background: Some(iced::Background::Color(bg)),
            text_color: tc,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

fn status_bar(app: &ConsoleApp) -> Element<'_, Message> {
    let iface = app
        .selected_interface
        .as_deref()
        .map(or_none)
        .unwrap_or("—");
    let mon = app.selected_monitor.as_deref().map(or_none).unwrap_or("—");
    let (svc_label, svc_color) = if app.network_services_killed {
        ("killed", DANGER_FG)
    } else {
        ("up", SUCCESS_FG)
    };
    let target = if app.target_ap.essid.is_empty() {
        "—"
    } else {
        app.target_ap.essid.as_str()
    };
    let (st_label, st_color) = if app.is_loading {
        ("busy", WARNING_FG)
    } else {
        ("idle", TEXT_MUTED)
    };

    let drag_zone = mouse_area(
        container(
            row![
                text("AngrySniffer").color(TEXT_PRIMARY).size(13),
                text("v0.1").color(TEXT_DIM).size(10),
                container(space::horizontal().width(1))
                    .width(1)
                    .height(12)
                    .style(|_: &Theme| container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(BORDER)),
                        border: Border::default(),
                        shadow: Shadow::default(),
                        snap: false,
                    }),
                status_item("iface", iface, TEXT_PRIMARY),
                status_item("mon", mon, TEXT_PRIMARY),
                status_item("services", svc_label, svc_color),
                status_item(
                    "target",
                    target,
                    if target == "—" {
                        TEXT_MUTED
                    } else {
                        TEXT_PRIMARY
                    },
                ),
                status_item("status", st_label, st_color),
                space::horizontal(),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .padding([0, 10])
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .on_press(Message::TitleBarDrag)
    .on_double_click(Message::TitleBarDoubleClick);

    let settings_btn = button(text("Settings").size(11))
        .style(btn_style(BtnKind::Ghost))
        .on_press(Message::OpenSettings)
        .padding([5, 10]);

    let controls = row![
        settings_btn,
        win_btn("—", WinBtn::Minimize),
        win_btn("□", WinBtn::Maximize),
        win_btn("×", WinBtn::Close),
    ]
    .spacing(0)
    .align_y(Alignment::Center);

    let bar = row![drag_zone, controls]
        .spacing(0)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fixed(32.0));

    container(bar)
        .style(header_bar)
        .width(Length::Fill)
        .into()
}

fn jobs_bar(app: &ConsoleApp) -> Element<'_, Message> {
    let mut chips: Vec<Element<'_, Message>> = Vec::new();
    chips.push(text("JOBS").color(TEXT_DIM).size(10).into());

    for job in &app.jobs {
        let selected = app.selected_job_id == Some(job.id);
        let id = job.id;
        let label = format!("{}:{}", job.kind.as_str(), job.label);
        let select_btn = button(text(label).size(10))
            .on_press(Message::SelectJob(id))
            .padding([3, 8])
            .style(move |_: &Theme, status: Status| -> button::Style {
                let bg = if selected {
                    TABLE_SELECTED
                } else if matches!(status, Status::Hovered) {
                    BG_BTN_HOVER
                } else {
                    BG_BTN
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    text_color: if selected { TEXT_PRIMARY } else { TEXT_MUTED },
                    border: Border {
                        color: if selected { ACCENT } else { BORDER },
                        width: 1.0,
                        radius: R.into(),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                }
            });
        let kill_btn = button(text("×").size(11).center())
            .on_press(Message::KillJob(id))
            .padding(0)
            .width(Length::Fixed(22.0))
            .height(Length::Fixed(22.0))
            .style(btn_style(BtnKind::Danger));
        chips.push(
            row![select_btn, kill_btn]
                .spacing(2)
                .align_y(Alignment::Center)
                .into(),
        );
    }

    chips.push(space::horizontal().into());
    if app.jobs.iter().any(|j| j.running) {
        chips.push(
            button(text("Kill all").size(10))
                .style(btn_style(BtnKind::Danger))
                .on_press(Message::KillAllJobs)
                .padding([3, 8])
                .into(),
        );
    }

    let detail = app
        .selected_job_id
        .and_then(|id| app.jobs.iter().find(|j| j.id == id))
        .map(|j| {
            format!(
                "#{} {} · pid {} · {}",
                j.id,
                j.kind.as_str(),
                j.pid,
                j.summary
            )
        })
        .unwrap_or_default();

    let row1 = row(chips)
        .spacing(6)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    let body = if detail.is_empty() {
        column![row1].spacing(2)
    } else {
        column![
            row1,
            text(detail).color(TEXT_DIM).size(10),
        ]
        .spacing(2)
    };

    container(body)
        .style(|_: &Theme| container::Style {
            text_color: None,
            background: Some(iced::Background::Color(BG_PANEL)),
            border: Border {
                color: BORDER_SUBTLE,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        })
        .padding([4, 10])
        .width(Length::Fill)
        .into()
}

fn title_bar_simple(subtitle: &str) -> Element<'_, Message> {
    let drag_zone = mouse_area(
        container(
            row![
                text("AngrySniffer").color(TEXT_PRIMARY).size(13),
                text(subtitle).color(TEXT_MUTED).size(11),
                space::horizontal(),
            ]
            .spacing(10)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .padding([0, 10])
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .on_press(Message::TitleBarDrag)
    .on_double_click(Message::TitleBarDoubleClick);

    let controls = row![
        win_btn("—", WinBtn::Minimize),
        win_btn("□", WinBtn::Maximize),
        win_btn("×", WinBtn::Close),
    ]
    .align_y(Alignment::Center);

    container(
        row![drag_zone, controls]
            .width(Length::Fill)
            .height(Length::Fixed(32.0))
            .align_y(Alignment::Center),
    )
    .style(header_bar)
    .width(Length::Fill)
    .into()
}

// ─── Sidebar (scrollable) ───────────────────────────────────────────────────

fn sidebar(app: &ConsoleApp) -> Element<'_, Message> {
    let target_summary = if app.target_ap.essid.is_empty() {
        text("No AP selected").color(TEXT_DIM).size(11)
    } else {
        text(format!(
            "{}  ·  ch {}  ·  {}",
            app.target_ap.essid, app.target_ap.channel, app.target_ap.bssid
        ))
        .color(TEXT_PRIMARY)
        .size(11)
    };

    let console_toggle = if app.show_console {
        compact_btn("Hide Console", BtnKind::Ghost, Message::ToggleConsole)
    } else {
        compact_btn("Show Console", BtnKind::Default, Message::ToggleConsole)
    };

    let controls = column![
        section_label("Interface"),
        action_btn("List Interfaces", BtnKind::Default, Message::ListInterfaces),
        pick_list(
            app.interfaces.clone(),
            app.selected_interface.clone(),
            |s| Message::InterfaceSelected(Some(s)),
        )
        .placeholder("Select interface")
        .width(Length::Fill)
        .style(picklist_style)
        .padding(6),
        pick_list(
            app.monitor_interfaces.clone(),
            app.selected_monitor.clone(),
            |s| Message::MonitorSelected(Some(s)),
        )
        .placeholder("Select monitor")
        .width(Length::Fill)
        .style(picklist_style)
        .padding(6),
        row![
            compact_btn("Add Monitor", BtnKind::Default, Message::AddMonitor),
            text_input("monitor name", &app.new_monitor_input)
                .on_input(Message::NewMonitorInputChanged)
                .padding(6)
                .style(input_style)
                .width(Length::Fill),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        row![
            pick_list(
                app.interfaces.clone(),
                Some(app.down_interface_input.clone()).filter(|s| !s.is_empty()),
                |s| Message::DownInterfaceSelected(Some(s)),
            )
            .placeholder("iface down")
            .width(Length::Fill)
            .style(picklist_style)
            .padding(6),
            compact_btn("Down", BtnKind::Default, Message::DownInterface),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        row![
            pick_list(
                app.interfaces.clone(),
                Some(app.up_interface_input.clone()).filter(|s| !s.is_empty()),
                |s| Message::UpInterfaceSelected(Some(s)),
            )
            .placeholder("iface up")
            .width(Length::Fill)
            .style(picklist_style)
            .padding(6),
            compact_btn("Up", BtnKind::Default, Message::UpInterface),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
        row![
            compact_btn("Kill Services", BtnKind::Danger, Message::KillNetworkServices),
            compact_btn("Lift Services", BtnKind::Default, Message::LiftNetworkServices),
        ]
        .spacing(6),
        action_btn(
            "Collect Network List",
            BtnKind::Default,
            Message::StartCollectingNetworkList,
        ),
        space::vertical().height(10),
        section_label("Target"),
        action_btn("Select AP File", BtnKind::Default, Message::SelectAPFile),
        target_summary,
        text_input("Station MAC", &app.station_mac)
            .on_input(Message::StationMacInputChanged)
            .padding(6)
            .style(input_style)
            .width(Length::Fill),
        space::vertical().height(10),
        section_label("Attack"),
        action_btn("Deauth Target", BtnKind::Danger, Message::DeauthTarget),
        action_btn("Start Capturing", BtnKind::Default, Message::StartCapturing),
        space::vertical().height(10),
        section_label("Cracking"),
        action_btn(
            "Crack Handshake",
            BtnKind::Default,
            Message::CrackCapturedHandshake,
        ),
        action_btn(
            "Crack Capture File",
            BtnKind::Default,
            Message::CrackCaptureFileLocally,
        ),
        space::vertical().height(12),
        console_toggle,
    ]
    .spacing(6)
    .padding(10)
    .width(Length::Fill);

    Scrollable::new(controls)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

// ─── Main panel ─────────────────────────────────────────────────────────────

fn main_panel(app: &ConsoleApp) -> Element<'_, Message> {
    let mut parts: Vec<Element<'_, Message>> = vec![ap_table_view(app)];

    if app.show_console {
        parts.push(console_view(app));
    }

    column(parts)
        .spacing(6)
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn console_view(app: &ConsoleApp) -> Element<'_, Message> {
    let header = row![
        text("Console").color(TEXT_MUTED).size(11),
        space::horizontal(),
        button(text("Hide").size(10))
            .style(btn_style(BtnKind::Ghost))
            .on_press(Message::ToggleConsole)
            .padding([3, 8]),
    ]
    .align_y(Alignment::Center)
    .width(Length::Fill);

    let body = Scrollable::new(text(&app.console_output).color(CONSOLE_FG).size(11))
        .id(app.scrollable_id.clone())
        .height(Length::Fill);

    let panel = column![header, divider(), body].spacing(4).height(Length::Fill);

    container(panel)
        .style(console_style)
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fixed(180.0))
        .into()
}

// ─── Settings ───────────────────────────────────────────────────────────────

fn settings_view(app: &ConsoleApp) -> Element<'_, Message> {
    let form = column![
        text("Settings").color(TEXT_PRIMARY).size(16),
        text("Paths, terminal emulator, and remote credentials.")
            .color(TEXT_MUTED)
            .size(12),
        space::vertical().height(12),
        settings_field(
            "Storage location",
            &app.storage_location_input,
            Message::StorageLocationInputChanged,
            Message::OpenStorageLocationDialog,
        ),
        settings_field(
            "Remote credentials",
            &app.remote_server_credentials_input,
            Message::RemoteServerCredentialsInputChanged,
            Message::OpenRemoteServerCredentialsDialog,
        ),
        settings_field(
            "Password list",
            &app.local_password_list_input,
            Message::LocalPasswordListInputChanged,
            Message::OpenLocalPasswordListDialog,
        ),
        column![
            text("Terminal emulator").color(TEXT_MUTED).size(11),
            text("Empty = auto-detect ($TERMINAL, then kitty/alacritty/…). Example: kitty")
                .color(TEXT_DIM)
                .size(10),
            text_input("auto", &app.terminal_input)
                .on_input(Message::TerminalInputChanged)
                .padding(7)
                .style(input_style)
                .width(Length::Fill),
        ]
        .spacing(4)
        .width(Length::Fill),
        space::vertical().height(16),
        row![
            button(text("Save").size(12))
                .style(btn_style(BtnKind::Default))
                .on_press(Message::SaveSettings)
                .padding([8, 14])
                .width(Length::Fill),
            button(text("Back").size(12))
                .style(btn_style(BtnKind::Ghost))
                .on_press(Message::CloseSettings)
                .padding([8, 14])
                .width(Length::Fill),
        ]
        .spacing(8)
        .width(Length::Fill),
    ]
    .spacing(10)
    .padding(24)
    .width(Length::Fill)
    .max_width(560);

    let panel = container(form).style(settings_panel).width(Length::Fill);

    let body = container(panel)
        .style(overlay_bg)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(24);

    column![title_bar_simple("Settings"), body]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn settings_field<'a>(
    label: &'a str,
    value: &'a str,
    on_input: fn(String) -> Message,
    on_browse: Message,
) -> Element<'a, Message> {
    column![
        text(label).color(TEXT_MUTED).size(11),
        row![
            text_input("Path", value)
                .on_input(on_input)
                .padding(7)
                .style(input_style)
                .width(Length::Fill),
            button(text("Browse").size(11))
                .style(btn_style(BtnKind::Default))
                .on_press(on_browse)
                .padding([7, 10]),
        ]
        .spacing(6)
        .align_y(Alignment::Center),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

// ─── AP table ───────────────────────────────────────────────────────────────

fn col_header(
    label: &str,
    col: usize,
    sort_col: usize,
    desc: bool,
    flex: u16,
) -> Element<'static, Message> {
    let arrow = if col == sort_col {
        if desc {
            " ▼"
        } else {
            " ▲"
        }
    } else {
        ""
    };
    button(text(format!("{}{}", label, arrow)).size(11))
        .on_press(Message::SortByColumn(col))
        .style(move |_: &Theme, status: Status| -> button::Style {
            let bg = match status {
                Status::Hovered => TABLE_HOVER,
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: if col == sort_col { TEXT_PRIMARY } else { TEXT_MUTED },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
                snap: false,
            }
        })
        .padding([3, 2])
        .width(Length::FillPortion(flex))
        .into()
}

fn ap_table_view(app: &ConsoleApp) -> Element<'_, Message> {
    let count_label = app.aps.len().to_string();

    let toolbar = row![
        text("Access Points").color(TEXT_PRIMARY).size(13),
        text(format!("({count_label})")).color(TEXT_MUTED).size(11),
        space::horizontal().width(Length::Fill),
        text_input("Filter ESSID…", &app.filter_text)
            .on_input(Message::FilterTextChanged)
            .padding(6)
            .style(input_style)
            .width(Length::Fixed(220.0)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
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
        .padding([5, 8])
        .align_y(Alignment::Center),
    )
    .style(header_row_style)
    .width(Length::Fill);

    let filtered: Vec<&AP> = if app.filter_text.is_empty() {
        app.aps.iter().collect()
    } else {
        let lower = app.filter_text.to_lowercase();
        app.aps
            .iter()
            .filter(|ap| ap.essid.to_lowercase().contains(&lower))
            .collect()
    };

    let display_aps = {
        let mut cloned: Vec<AP> = filtered.iter().map(|ap| (*ap).clone()).collect();
        if app.sort_column > 0 || app.sort_descending {
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
        }
        cloned
    };

    let mut rows: Vec<Element<'_, Message>> = Vec::new();

    if display_aps.is_empty() {
        let empty_msg = if app.aps.is_empty() {
            "No access points loaded. Collect a network list or select an AP CSV file."
        } else {
            "No APs match the current filter."
        };
        rows.push(
            container(text(empty_msg).color(TEXT_MUTED).size(12))
                .width(Length::Fill)
                .padding(28)
                .center_x(Length::Fill)
                .into(),
        );
    }

    for (i, ap) in display_aps.iter().enumerate() {
        let bssid = ap.bssid.clone();
        let power = format!("{} dBm", ap.power);
        let channel = format!("{}", ap.channel);
        let essid = if ap.essid.is_empty() {
            "<hidden>".to_string()
        } else {
            ap.essid.clone()
        };
        let privacy = ap.privacy.clone();
        let is_selected = app.selected_n < app.aps.len()
            && ap.bssid == app.aps[app.selected_n].bssid
            && ap.essid == app.aps[app.selected_n].essid;

        let power_color = if ap.power > -50 {
            POWER_STRONG
        } else if ap.power > -70 {
            POWER_MID
        } else {
            POWER_WEAK
        };

        let privacy_color = match privacy.as_str() {
            "WPA2" | "WPA3" => SUCCESS_FG,
            "WPA" => WARNING_FG,
            "WEP" | "OPN" => DANGER_FG,
            _ => TEXT_MUTED,
        };

        let essid_color = if essid == "<hidden>" {
            TEXT_MUTED
        } else {
            TEXT_PRIMARY
        };
        let idx_color = if is_selected { TEXT_PRIMARY } else { TEXT_DIM };

        let actual_idx = app
            .aps
            .iter()
            .position(|a| a.bssid == ap.bssid && a.essid == ap.essid)
            .unwrap_or(usize::MAX);

        let row_entry = button(
            row![
                text(format!("{i}"))
                    .color(idx_color)
                    .size(11)
                    .width(Length::FillPortion(1)),
                text(bssid)
                    .color(TEXT_PRIMARY)
                    .size(11)
                    .width(Length::FillPortion(4)),
                text(power)
                    .color(power_color)
                    .size(11)
                    .width(Length::FillPortion(2)),
                text(channel)
                    .color(TEXT_PRIMARY)
                    .size(11)
                    .width(Length::FillPortion(1)),
                text(essid)
                    .color(essid_color)
                    .size(11)
                    .width(Length::FillPortion(4)),
                text(privacy)
                    .color(privacy_color)
                    .size(11)
                    .width(Length::FillPortion(2)),
            ]
            .spacing(4)
            .padding(iced::Padding::from([4, 8]))
            .align_y(Alignment::Center),
        )
        .on_press(Message::SelectApFromTable(actual_idx))
        .style(move |_: &Theme, status: Status| -> button::Style {
            let bg = match status {
                Status::Hovered => TABLE_HOVER,
                _ if is_selected => TABLE_SELECTED,
                _ if i % 2 == 0 => TABLE_ROW_ALT,
                _ => BG_APP,
            };
            button::Style {
                background: Some(iced::Background::Color(bg)),
                text_color: TEXT_PRIMARY,
                border: Border {
                    color: if is_selected { ACCENT } else { Color::TRANSPARENT },
                    width: if is_selected { 1.0 } else { 0.0 },
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

    let summary: Element<'_, Message> = if !display_aps.is_empty() && app.selected_n < app.aps.len()
    {
        text(format!(
            "{} APs · filtered {} · selected {}",
            app.aps.len(),
            if app.filter_text.is_empty() {
                "all".to_string()
            } else {
                display_aps.len().to_string()
            },
            app.aps[app.selected_n].essid
        ))
        .color(TEXT_MUTED)
        .size(10)
        .into()
    } else if !app.aps.is_empty() {
        text(format!(
            "{} APs · showing {}",
            app.aps.len(),
            display_aps.len()
        ))
        .color(TEXT_MUTED)
        .size(10)
        .into()
    } else {
        text("Load a CSV from Collect or Select AP File")
            .color(TEXT_DIM)
            .size(10)
            .into()
    };

    let mut table_content: Vec<Element<'_, Message>> = Vec::with_capacity(rows.len() + 1);
    table_content.push(header.into());
    table_content.extend(rows);

    let table_scroll = Scrollable::new(column(table_content).spacing(0)).height(Length::Fill);

    let table = column![toolbar, table_scroll, summary]
        .spacing(6)
        .height(Length::Fill);

    container(table)
        .style(panel_style)
        .padding(10)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}
