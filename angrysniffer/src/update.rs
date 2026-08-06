use crate::calllib::parse_network_list;
use crate::message::Message;
use crate::state::{ConsoleApp, Job, JobKind};
use commands::{
    ensure_trailing_slash, find_capture_for_prefix, get_interface_names, get_monitor_interfaces,
    kill_process_group, resolve_terminal, run_command, run_sudo_command, sanitize_filename,
    spawn_terminal_job, wait_child,
};
use iced::widget::operation;
use iced::window;
use iced::Task;
use nix::unistd::geteuid;
use std::process::Output;
use std::sync::Arc;

pub mod commands;

fn with_window(f: impl Fn(window::Id) -> Task<Message> + Send + Sync + 'static) -> Task<Message> {
    window::latest().then(move |id| match id {
        Some(id) => f(id),
        None => Task::none(),
    })
}

fn log_line(app: &mut ConsoleApp, tag: &str, msg: &str) {
    if !app.console_output.is_empty() && !app.console_output.ends_with('\n') {
        app.console_output.push('\n');
    }
    app.console_output.push_str(&format!("[{tag}] {msg}\n"));
}

fn log_info(app: &mut ConsoleApp, msg: impl AsRef<str>) {
    log_line(app, "info", msg.as_ref());
}

fn log_ok(app: &mut ConsoleApp, msg: impl AsRef<str>) {
    log_line(app, " ok ", msg.as_ref());
}

fn log_err(app: &mut ConsoleApp, msg: impl AsRef<str>) {
    log_line(app, "err ", msg.as_ref());
    app.show_console = true;
}

fn log_cmd(app: &mut ConsoleApp, msg: impl AsRef<str>) {
    log_line(app, "cmd ", msg.as_ref());
}

fn log_warn(app: &mut ConsoleApp, msg: impl AsRef<str>) {
    log_line(app, "warn", msg.as_ref());
    app.show_console = true;
}

fn log_job(app: &mut ConsoleApp, msg: impl AsRef<str>) {
    log_line(app, "job ", msg.as_ref());
}

fn snap_console(app: &ConsoleApp) -> Task<Message> {
    operation::snap_to(app.scrollable_id.clone(), operation::RelativeOffset::END)
}

fn refresh_interfaces(app: &mut ConsoleApp) {
    app.interfaces = get_interface_names();
    app.monitor_interfaces = get_monitor_interfaces();
    if let Some(ref sel) = app.selected_interface {
        if !app.interfaces.iter().any(|i| i == sel) {
            app.selected_interface = None;
        }
    }
    if let Some(ref sel) = app.selected_monitor {
        if !app.monitor_interfaces.iter().any(|i| i == sel) {
            app.selected_monitor = None;
        }
    }
}

fn storage_dir(app: &ConsoleApp) -> String {
    ensure_trailing_slash(if !app.storage_location.is_empty() {
        &app.storage_location
    } else {
        &app.path_to_network
    })
}

fn wordlist(app: &ConsoleApp) -> String {
    if !app.local_password_list_input.is_empty() {
        app.local_password_list_input.clone()
    } else {
        app.local_password_list.clone()
    }
}

fn valid_iface(name: &str) -> bool {
    !name.is_empty() && name != "none"
}

fn require_monitor(app: &mut ConsoleApp) -> Option<String> {
    match app.selected_monitor.as_deref() {
        Some(m) if valid_iface(m) => Some(m.to_string()),
        _ => {
            log_err(app, "Select a monitor interface first.");
            None
        }
    }
}

fn require_target(app: &mut ConsoleApp) -> bool {
    if app.target_ap.has_target() {
        true
    } else {
        log_err(app, "Select a target AP from the table first.");
        false
    }
}

fn require_wordlist(app: &mut ConsoleApp) -> Option<String> {
    let wl = wordlist(app);
    if wl.is_empty() {
        log_err(app, "Set a password list in Settings first.");
        return None;
    }
    if !std::path::Path::new(&wl).is_file() {
        log_err(app, format!("Password list not found: {wl}"));
        return None;
    }
    Some(wl)
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Prefix a shell command with sudo when not already root.
/// Empty password: prefer non-interactive (`-n`), else interactive `sudo` in the terminal
/// (NOPASSWD, cached ticket, or user types password / blank Enter).
fn elevate_shell(app: &ConsoleApp, inner: &str) -> String {
    if geteuid().is_root() {
        return inner.to_string();
    }
    if app.sudo_password.is_empty() {
        format!(
            "if sudo -n true >/dev/null 2>&1; then sudo -n {inner}; else sudo {inner}; fi"
        )
    } else {
        format!(
            "echo {} | sudo -S {inner}",
            shell_quote(&app.sudo_password)
        )
    }
}

fn run_labeled(
    label: &str,
    external: bool,
    refresh_ifaces: bool,
    fut: impl std::future::Future<Output = Result<Output, Arc<std::io::Error>>> + Send + 'static,
) -> Task<Message> {
    let label = label.to_string();
    Task::perform(fut, move |result| Message::CommandCompleted {
        label: label.clone(),
        external,
        refresh_ifaces,
        result,
    })
}

fn run_priv(
    app: &ConsoleApp,
    label: &str,
    refresh_ifaces: bool,
    command: String,
    args: Vec<String>,
) -> Task<Message> {
    if geteuid().is_root() {
        run_labeled(label, false, refresh_ifaces, run_command(command, args))
    } else {
        let pw = app.sudo_password.clone();
        run_labeled(
            label,
            false,
            refresh_ifaces,
            run_sudo_command(command, args, pw),
        )
    }
}

fn zenity_pick(
    title: &str,
    filename: &str,
    directory: bool,
    on_pick: fn(String) -> Message,
) -> Task<Message> {
    let title_s = title.to_string();
    let mut args = vec![
        "--file-selection".to_string(),
        format!("--title={title}"),
        format!("--filename={filename}"),
    ];
    if directory {
        args.push("--directory".to_string());
    }
    Task::perform(run_command("zenity".to_string(), args), move |result| {
        match result {
            Ok(output) => {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if path.is_empty() || !output.status.success() {
                    Message::DialogCancelled(title_s.clone())
                } else {
                    on_pick(path)
                }
            }
            Err(e) => Message::CommandCompleted {
                label: format!("dialog:{title_s}"),
                external: false,
                refresh_ifaces: false,
                result: Err(e),
            },
        }
    })
}

fn zenity_file(title: &str, filename: &str, on_pick: fn(String) -> Message) -> Task<Message> {
    zenity_pick(title, filename, false, on_pick)
}

fn zenity_dir(title: &str, filename: &str, on_pick: fn(String) -> Message) -> Task<Message> {
    zenity_pick(title, filename, true, on_pick)
}

fn start_terminal_job(
    app: &mut ConsoleApp,
    kind: JobKind,
    label: impl Into<String>,
    summary: impl Into<String>,
    script: String,
) -> Task<Message> {
    let label = label.into();
    let summary = summary.into();
    app.show_console = true;

    match spawn_terminal_job(&app.terminal, &script) {
        Ok(spawned) => {
            let id = app.next_job_id;
            app.next_job_id = app.next_job_id.wrapping_add(1);
            app.jobs.push(Job {
                id,
                kind,
                label: label.clone(),
                summary: summary.clone(),
                pid: spawned.pid,
                pgid: spawned.pgid,
                running: true,
            });
            app.selected_job_id = Some(id);
            log_job(
                app,
                format!(
                    "#{id} {} started — {label} (pid {}, via {})",
                    kind.as_str(),
                    spawned.pid,
                    spawned.terminal
                ),
            );
            if !summary.is_empty() {
                log_info(app, &summary);
            }
            Task::perform(wait_child(spawned.child), move |result| Message::JobFinished {
                id,
                result,
            })
        }
        Err(e) => {
            log_err(app, format!("Could not start {}: {e}", kind.as_str()));
            snap_console(app)
        }
    }
}

fn kill_job_by_id(app: &mut ConsoleApp, id: u64) -> bool {
    let Some((kind, label, pgid)) = app
        .jobs
        .iter()
        .find(|j| j.id == id && j.running)
        .map(|j| (j.kind.as_str(), j.label.clone(), j.pgid))
    else {
        return false;
    };
    log_job(app, format!("#{id} killing {kind} ({label}) pgid {pgid}"));
    kill_process_group(pgid);
    true
}

fn kill_all_jobs(app: &mut ConsoleApp) {
    let ids: Vec<u64> = app
        .jobs
        .iter()
        .filter(|j| j.running)
        .map(|j| j.id)
        .collect();
    for id in ids {
        kill_job_by_id(app, id);
    }
}

fn start_aircrack(app: &mut ConsoleApp, cap_path: &str, wl: &str) -> Task<Message> {
    app.cap_file_path = cap_path.to_string();
    let crack = format!(
        "aircrack-ng {} -w {}",
        shell_quote(cap_path),
        shell_quote(wl)
    );
    let body = format!(
        "echo '=== aircrack-ng ==='; echo 'cap: {}'; echo 'wordlist: {}'; echo; {}; echo; echo 'Press Enter to close...'; read",
        cap_path, wl, crack
    );
    log_cmd(app, &crack);
    let short = std::path::Path::new(cap_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| cap_path.to_string());
    start_terminal_job(
        app,
        JobKind::Crack,
        short,
        format!("Cracking {cap_path}"),
        body,
    )
}

pub fn update(app: &mut ConsoleApp, message: Message) -> Task<Message> {
    match message {
        Message::StationMacInputChanged(value) => {
            app.station_mac = value;
            Task::none()
        }
        Message::NewMonitorInputChanged(value) => {
            app.new_monitor_input = value;
            Task::none()
        }
        Message::InterfaceSelected(selected) => {
            app.selected_interface = selected.filter(|s| valid_iface(s));
            if let Some(ref iface) = app.selected_interface {
                log_info(app, format!("Interface selected: {iface}"));
            }
            snap_console(app)
        }
        Message::MonitorSelected(selected) => {
            app.selected_monitor = selected.filter(|s| valid_iface(s));
            if let Some(ref mon) = app.selected_monitor {
                log_info(app, format!("Monitor selected: {mon}"));
            }
            snap_console(app)
        }
        Message::DownInterfaceSelected(selected) => {
            if let Some(interface) = selected.filter(|s| valid_iface(s)) {
                app.down_interface_input = interface;
            }
            Task::none()
        }
        Message::UpInterfaceSelected(selected) => {
            if let Some(interface) = selected.filter(|s| valid_iface(s)) {
                app.up_interface_input = interface;
            }
            Task::none()
        }
        Message::ListInterfaces => {
            refresh_interfaces(app);
            log_info(app, "Refreshing interface lists…");
            log_cmd(app, "ip a");
            app.is_loading = true;
            app.show_console = true;
            run_labeled(
                "list interfaces",
                false,
                true,
                run_command("ip".into(), vec!["a".into()]),
            )
        }
        Message::AddMonitor => {
            let Some(iface) = app.selected_interface.clone().filter(|s| valid_iface(s)) else {
                log_err(app, "Select a base interface before adding a monitor.");
                return snap_console(app);
            };
            let mon_name = app.new_monitor_input.trim().to_string();
            if mon_name.is_empty() {
                log_err(app, "Enter a monitor interface name (e.g. wlan0mon).");
                return snap_console(app);
            }
            log_cmd(
                app,
                format!("iw dev {iface} interface add {mon_name} type monitor"),
            );
            app.is_loading = true;
            app.show_console = true;
            run_priv(
                app,
                "add monitor",
                true,
                "iw".into(),
                vec![
                    "dev".into(),
                    iface,
                    "interface".into(),
                    "add".into(),
                    mon_name,
                    "type".into(),
                    "monitor".into(),
                ],
            )
        }
        Message::DownInterface => {
            let iface = app.down_interface_input.trim().to_string();
            if !valid_iface(&iface) {
                log_err(app, "Select an interface to bring down.");
                return snap_console(app);
            }
            log_cmd(app, format!("ip link set {iface} down"));
            app.is_loading = true;
            app.show_console = true;
            run_priv(
                app,
                "iface down",
                true,
                "ip".into(),
                vec!["link".into(), "set".into(), iface, "down".into()],
            )
        }
        Message::UpInterface => {
            let iface = app.up_interface_input.trim().to_string();
            if !valid_iface(&iface) {
                log_err(app, "Select an interface to bring up.");
                return snap_console(app);
            }
            log_cmd(app, format!("ip link set {iface} up"));
            app.is_loading = true;
            app.show_console = true;
            run_priv(
                app,
                "iface up",
                true,
                "ip".into(),
                vec!["link".into(), "set".into(), iface, "up".into()],
            )
        }
        Message::KillNetworkServices => {
            log_cmd(app, "airmon-ng check kill");
            log_warn(
                app,
                "Stopping NetworkManager / wpa_supplicant (airmon-ng check kill).",
            );
            app.is_loading = true;
            app.show_console = true;
            run_priv(
                app,
                "kill network services",
                false,
                "airmon-ng".into(),
                vec!["check".into(), "kill".into()],
            )
        }
        Message::LiftNetworkServices => {
            log_cmd(
                app,
                "systemctl restart NetworkManager.service wpa_supplicant.service",
            );
            app.is_loading = true;
            app.show_console = true;
            run_priv(
                app,
                "lift network services",
                false,
                "systemctl".into(),
                vec![
                    "restart".into(),
                    "NetworkManager.service".into(),
                    "wpa_supplicant.service".into(),
                ],
            )
        }
        Message::StartCollectingNetworkList => {
            let Some(mon) = require_monitor(app) else {
                return snap_console(app);
            };
            let dir = storage_dir(app);
            if let Err(e) = std::fs::create_dir_all(&dir) {
                log_err(app, format!("Cannot create storage dir {dir}: {e}"));
                return snap_console(app);
            }
            let prefix = format!("{dir}scan");
            let dump = format!(
                "airodump-ng {mon} --output-format csv -w {}",
                shell_quote(&prefix)
            );
            let script = format!(
                "echo '=== airodump-ng (network list) ==='; echo 'iface: {mon}'; echo 'out: {prefix}*.csv'; echo 'Close this terminal when done.'; echo; {}; echo; echo 'Press Enter to close...'; read",
                elevate_shell(app, &dump)
            );
            log_cmd(app, &dump);
            start_terminal_job(
                app,
                JobKind::Scan,
                mon.clone(),
                format!("Network scan on {mon} → {dir}scan-*.csv"),
                script,
            )
        }
        Message::SelectAPFile => {
            let dir = storage_dir(app);
            log_info(app, "Select an airodump CSV network list…");
            app.is_loading = true;
            app.show_console = true;
            zenity_file("Select Target AP File", &dir, Message::SetPathToApFile)
        }
        Message::DeauthTarget => {
            if !require_target(app) {
                return snap_console(app);
            }
            let Some(mon) = require_monitor(app) else {
                return snap_console(app);
            };
            let sta = app.station_mac.trim().to_string();
            if sta.len() != 17 {
                log_err(
                    app,
                    "Station MAC must look like AA:BB:CC:DD:EE:FF (17 chars).",
                );
                return snap_console(app);
            }
            let bssid = app.target_ap.bssid.clone();
            let essid = if app.target_ap.essid.is_empty() {
                bssid.clone()
            } else {
                app.target_ap.essid.clone()
            };
            let deauth = format!("aireplay-ng --deauth 10 -a {bssid} -c {sta} {mon}");
            let script = format!(
                "echo '=== aireplay-ng deauth ==='; echo 'AP: {bssid}'; echo 'STA: {sta}'; echo 'iface: {mon}'; echo; {}; echo; echo 'Press Enter to close...'; read",
                elevate_shell(app, &deauth)
            );
            log_cmd(app, &deauth);
            start_terminal_job(
                app,
                JobKind::Deauth,
                essid,
                format!("Deauth {bssid} → station {sta}"),
                script,
            )
        }
        Message::StartCapturing => {
            if !require_target(app) {
                return snap_console(app);
            }
            let Some(mon) = require_monitor(app) else {
                return snap_console(app);
            };
            let dir = storage_dir(app);
            if let Err(e) = std::fs::create_dir_all(&dir) {
                log_err(app, format!("Cannot create storage dir {dir}: {e}"));
                return snap_console(app);
            }
            let essid_safe = sanitize_filename(&app.target_ap.essid);
            let bssid = app.target_ap.bssid.clone();
            let ch = app.target_ap.channel;
            let prefix = format!("{dir}{essid_safe}");
            let essid_label = if app.target_ap.essid.is_empty() {
                bssid.clone()
            } else {
                app.target_ap.essid.clone()
            };
            let dump = format!(
                "airodump-ng --bssid {bssid} -c {ch} {mon} --output-format cap -w {}",
                shell_quote(&prefix)
            );
            let script = format!(
                "echo '=== airodump-ng (capture) ==='; echo 'BSSID: {bssid}'; echo 'channel: {ch}'; echo 'iface: {mon}'; echo 'prefix: {prefix}'; echo 'Close terminal to stop capture.'; echo; {}; echo; echo 'Press Enter to close...'; read",
                elevate_shell(app, &dump)
            );
            log_cmd(app, &dump);
            start_terminal_job(
                app,
                JobKind::Capture,
                essid_label,
                format!("Capture → {prefix}-01.cap"),
                script,
            )
        }
        Message::CommandCompleted {
            label,
            external,
            refresh_ifaces,
            result,
        } => {
            app.is_loading = false;
            match result {
                Ok(output) => {
                    let code = output.status.code().unwrap_or(-1);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout_t = stdout.trim();
                    let stderr_t = stderr.trim();

                    if external {
                        if output.status.success() {
                            log_ok(
                                app,
                                format!("{label}: external terminal finished (exit {code})."),
                            );
                        } else {
                            log_warn(
                                app,
                                format!(
                                    "{label}: external terminal exited {code}.{}",
                                    if stderr_t.is_empty() {
                                        String::new()
                                    } else {
                                        format!(" stderr: {stderr_t}")
                                    }
                                ),
                            );
                        }
                    } else if output.status.success() {
                        log_ok(app, format!("{label}: success (exit {code})."));
                        if !stdout_t.is_empty() {
                            for line in stdout_t.lines().take(40) {
                                log_line(app, "out ", line);
                            }
                            if stdout_t.lines().count() > 40 {
                                log_info(app, "… stdout truncated …");
                            }
                        }
                        if !stderr_t.is_empty() {
                            for line in stderr_t.lines().take(20) {
                                log_line(app, "err ", line);
                            }
                        }
                        if label == "kill network services" {
                            app.network_services_killed = true;
                            log_warn(
                                app,
                                "Network services marked killed. Use Lift Services when done.",
                            );
                        }
                        if label == "lift network services" {
                            app.network_services_killed = false;
                            log_ok(app, "Network services restarted.");
                        }
                    } else {
                        log_err(app, format!("{label}: failed (exit {code})."));
                        if !stderr_t.is_empty() {
                            for line in stderr_t.lines().take(30) {
                                log_line(app, "err ", line);
                            }
                        }
                        if !stdout_t.is_empty() {
                            for line in stdout_t.lines().take(20) {
                                log_line(app, "out ", line);
                            }
                        }
                    }
                }
                Err(e) => {
                    log_err(app, format!("{label}: could not run — {e}"));
                }
            }
            if refresh_ifaces {
                refresh_interfaces(app);
                log_info(
                    app,
                    format!(
                        "Interfaces: {} | monitors: {}",
                        app.interfaces.len(),
                        app.monitor_interfaces.len()
                    ),
                );
            }
            snap_console(app)
        }
        Message::DialogCancelled(what) => {
            app.is_loading = false;
            log_info(app, format!("{what}: cancelled."));
            snap_console(app)
        }
        Message::SetPathToApFile(path) => {
            app.is_loading = false;
            app.path_to_csv_network = path.clone();
            match parse_network_list(&path) {
                Ok(aps) => {
                    app.aps = aps;
                    log_ok(app, format!("Loaded {} AP(s) from {path}", app.aps.len()));
                    if app.aps.is_empty() {
                        log_warn(app, "CSV contained no usable APs (check file format).");
                    } else {
                        let previews: Vec<String> = app
                            .aps
                            .iter()
                            .take(15)
                            .enumerate()
                            .map(|(i, ap)| format!("{i:>3}  {}", ap.summary()))
                            .collect();
                        let extra = app.aps.len().saturating_sub(15);
                        app.selected_n = 0;
                        app.target_ap = app.aps[0].clone();
                        app.selected_str = "0".into();
                        for line in previews {
                            log_line(app, " ap ", &line);
                        }
                        if extra > 0 {
                            log_info(app, format!("… and {extra} more (see table)."));
                        }
                        log_info(
                            app,
                            format!("Auto-selected #0: {}", app.target_ap.summary()),
                        );
                    }
                }
                Err(e) => {
                    app.aps.clear();
                    log_err(app, e);
                }
            }
            snap_console(app)
        }
        Message::OpenSettings => {
            app.show_settings = true;
            Task::none()
        }
        Message::CloseSettings => {
            app.show_settings = false;
            Task::none()
        }
        Message::OpenStorageLocationDialog => {
            app.is_loading = true;
            zenity_dir(
                "Select Default Storage Location",
                &storage_dir(app),
                Message::SetStorageLocation,
            )
        }
        Message::SetStorageLocation(path) => {
            app.is_loading = false;
            app.storage_location_input = path.clone();
            log_info(app, format!("Storage path set (unsaved): {path}"));
            snap_console(app)
        }
        Message::StorageLocationInputChanged(value) => {
            app.storage_location_input = value;
            Task::none()
        }
        Message::OpenRemoteServerCredentialsDialog => {
            app.is_loading = true;
            zenity_file(
                "Select Remote Server Credentials File",
                "./",
                Message::SetRemoteServerCredentials,
            )
        }
        Message::SetRemoteServerCredentials(path) => {
            app.is_loading = false;
            app.remote_server_credentials_input = path.clone();
            log_info(app, format!("Remote credentials path set (unsaved): {path}"));
            snap_console(app)
        }
        Message::RemoteServerCredentialsInputChanged(value) => {
            app.remote_server_credentials_input = value;
            Task::none()
        }
        Message::OpenLocalPasswordListDialog => {
            app.is_loading = true;
            zenity_file(
                "Select Local Password List File",
                "./",
                Message::SetLocalPasswordList,
            )
        }
        Message::SetLocalPasswordList(path) => {
            app.is_loading = false;
            app.local_password_list_input = path.clone();
            log_info(app, format!("Password list set (unsaved): {path}"));
            snap_console(app)
        }
        Message::LocalPasswordListInputChanged(value) => {
            app.local_password_list_input = value;
            Task::none()
        }
        Message::TerminalInputChanged(value) => {
            app.terminal_input = value;
            Task::none()
        }
        Message::SaveSettings => {
            let storage = ensure_trailing_slash(&app.storage_location_input);
            app.storage_location_input = storage.clone();
            let term = app.terminal_input.trim().to_string();
            if !term.is_empty() {
                if let Err(e) = resolve_terminal(&term) {
                    log_err(app, e);
                    return snap_console(app);
                }
            }
            let config_path = "./angrysniffer.toml";
            let config_content = format!(
                "# AngrySniffer Configuration\n[settings]\nstorage_location = \"{}\"\nremote_server_credentials = \"{}\"\nlocal_password_list = \"{}\"\nterminal = \"{}\"\n",
                storage,
                app.remote_server_credentials_input,
                app.local_password_list_input,
                term,
            );
            match std::fs::write(config_path, config_content) {
                Ok(_) => {
                    app.storage_location = storage.clone();
                    app.path_to_network = storage.clone();
                    app.remote_server_credentials = app.remote_server_credentials_input.clone();
                    app.local_password_list = app.local_password_list_input.clone();
                    app.terminal = term.clone();
                    app.terminal_input = term.clone();
                    if let Err(e) = std::fs::create_dir_all(&storage) {
                        log_warn(
                            app,
                            format!("Settings saved, but could not create {storage}: {e}"),
                        );
                    } else {
                        log_ok(app, format!("Settings saved → {config_path}"));
                        log_info(app, format!("Storage: {storage}"));
                        log_info(app, format!("Wordlist: {}", app.local_password_list));
                        match resolve_terminal(&app.terminal) {
                            Ok(t) => log_info(app, format!("Terminal: {t}")),
                            Err(e) => log_warn(app, e),
                        }
                    }
                    app.show_settings = false;
                }
                Err(e) => log_err(app, format!("Failed to save settings: {e}")),
            }
            snap_console(app)
        }
        Message::CrackCaptureFileLocally => {
            let Some(_wl) = require_wordlist(app) else {
                return snap_console(app);
            };
            let dir = storage_dir(app);
            log_info(app, "Select a .cap capture file to crack…");
            app.is_loading = true;
            app.show_console = true;
            zenity_file("Select capture file", &dir, Message::SetCapFilePathAndCrack)
        }
        Message::SetCapFilePathAndCrack(path) => {
            app.is_loading = false;
            if !std::path::Path::new(&path).is_file() {
                log_err(app, format!("Capture file not found: {path}"));
                return snap_console(app);
            }
            let Some(wl) = require_wordlist(app) else {
                return snap_console(app);
            };
            start_aircrack(app, &path, &wl)
        }
        Message::CrackCapturedHandshake => {
            if !require_target(app) {
                return snap_console(app);
            }
            let Some(wl) = require_wordlist(app) else {
                return snap_console(app);
            };
            let dir = storage_dir(app);
            let essid_safe = sanitize_filename(&app.target_ap.essid);
            let prefix = format!("{dir}{essid_safe}");
            match find_capture_for_prefix(&prefix) {
                Some(cap) => {
                    log_ok(
                        app,
                        format!(
                            "Found capture for '{}': {cap}",
                            if app.target_ap.essid.is_empty() {
                                &app.target_ap.bssid
                            } else {
                                &app.target_ap.essid
                            }
                        ),
                    );
                    start_aircrack(app, &cap, &wl)
                }
                None => {
                    log_err(
                        app,
                        format!(
                            "No .cap found for prefix {prefix}-*.cap. Run Start Capturing first, or use Crack Capture File."
                        ),
                    );
                    snap_console(app)
                }
            }
        }
        Message::SelectApFromTable(index) => {
            if index < app.aps.len() {
                app.selected_n = index;
                app.target_ap = app.aps[index].clone();
                app.selected_str = index.to_string();
                log_info(
                    app,
                    format!("Selected AP #{index}: {}", app.target_ap.summary()),
                );
            } else {
                log_err(app, format!("Invalid AP index: {index}"));
            }
            snap_console(app)
        }
        Message::ToggleConsole => {
            app.show_console = !app.show_console;
            Task::none()
        }
        Message::SortByColumn(col) => {
            if app.sort_column == col {
                app.sort_descending = !app.sort_descending;
            } else {
                app.sort_column = col;
                app.sort_descending = false;
            }
            Task::none()
        }
        Message::FilterTextChanged(text) => {
            app.filter_text = text;
            Task::none()
        }
        Message::SelectJob(id) => {
            if app.jobs.iter().any(|j| j.id == id) {
                app.selected_job_id = Some(id);
                if let Some(job) = app.jobs.iter().find(|j| j.id == id) {
                    log_job(
                        app,
                        format!(
                            "#{id} selected — {} {} | {}",
                            job.kind.as_str(),
                            job.label,
                            job.summary
                        ),
                    );
                }
            }
            snap_console(app)
        }
        Message::KillJob(id) => {
            if !kill_job_by_id(app, id) {
                log_warn(app, format!("Job #{id} is not running."));
            }
            snap_console(app)
        }
        Message::KillAllJobs => {
            let n = app.jobs.iter().filter(|j| j.running).count();
            if n == 0 {
                log_info(app, "No running jobs.");
            } else {
                log_job(app, format!("Killing {n} job(s)…"));
                kill_all_jobs(app);
            }
            snap_console(app)
        }
        Message::JobFinished { id, result } => {
            let kind_label = app
                .jobs
                .iter()
                .find(|j| j.id == id)
                .map(|j| format!("{} {}", j.kind.as_str(), j.label))
                .unwrap_or_else(|| format!("#{id}"));
            match result {
                Ok(code) => {
                    let code = code.unwrap_or(-1);
                    if code == 0 {
                        log_ok(app, format!("Job {kind_label} finished (exit {code})."));
                    } else {
                        log_warn(app, format!("Job {kind_label} exited ({code})."));
                    }
                }
                Err(e) => log_err(app, format!("Job {kind_label}: {e}")),
            }
            app.jobs.retain(|j| j.id != id);
            if app.selected_job_id == Some(id) {
                app.selected_job_id = app.jobs.last().map(|j| j.id);
            }
            snap_console(app)
        }
        Message::TitleBarDrag => with_window(window::drag),
        Message::TitleBarDoubleClick | Message::WindowToggleMaximize => {
            with_window(window::toggle_maximize)
        }
        Message::WindowMinimize => with_window(|id| window::minimize(id, true)),
        Message::WindowClose => {
            kill_all_jobs(app);
            with_window(window::close)
        }
    }
}
