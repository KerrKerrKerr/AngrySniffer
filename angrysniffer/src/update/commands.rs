use std::path::Path;
use std::process::Output;
use std::sync::Arc;
use std::time::Duration;

pub fn get_interface_names() -> Vec<String> {
    match std::fs::read_dir("/sys/class/net") {
        Ok(entries) => {
            let mut ifaces: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|n| n != "lo")
                .collect();
            ifaces.sort();
            ifaces
        }
        Err(_) => Vec::new(),
    }
}

pub fn get_monitor_interfaces() -> Vec<String> {
    match std::fs::read_dir("/sys/class/net") {
        Ok(entries) => {
            let mut monitor_ifaces: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|name| name.contains("mon") || is_monitor_interface(name))
                .collect();
            monitor_ifaces.sort();
            monitor_ifaces
        }
        Err(_) => Vec::new(),
    }
}

pub fn is_monitor_interface(interface_name: &str) -> bool {
    let type_path = format!("/sys/class/net/{interface_name}/type");
    std::fs::read_to_string(type_path)
        .ok()
        .and_then(|c| c.trim().parse::<u32>().ok())
        .map(|n| n == 803)
        .unwrap_or(false)
}

pub fn sanitize_filename(s: &str) -> String {
    let out: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
        .collect();
    if out.is_empty() {
        "capture".to_string()
    } else {
        out
    }
}

pub fn ensure_trailing_slash(path: &str) -> String {
    if path.is_empty() {
        return String::from("/root/.scans/");
    }
    if path.ends_with('/') {
        path.to_string()
    } else {
        format!("{path}/")
    }
}

/// Find newest matching capture for airodump prefix: `{prefix}-NN.cap`
pub fn find_capture_for_prefix(prefix_path: &str) -> Option<String> {
    let path = std::path::Path::new(prefix_path);
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let base = path.file_name()?.to_string_lossy().to_string();

    let mut matches: Vec<(std::time::SystemTime, String)> = Vec::new();
    let entries = std::fs::read_dir(parent).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(&format!("{base}-")) && name.ends_with(".cap") {
            let modified = entry
                .metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            matches.push((modified, entry.path().display().to_string()));
        }
    }
    matches.sort_by(|a, b| b.0.cmp(&a.0));
    matches.into_iter().next().map(|(_, p)| p)
}

/// Prompt the user for the sudo password via zenity (password mode, hidden input).
/// Returns the password string (without trailing newline).
pub fn prompt_sudo_password() -> String {
    match std::process::Command::new("zenity")
        .args([
            "--password",
            "--title",
            "AngrySniffer — Sudo Password",
            "--text",
            "Enter your sudo password (not stored on disk):",
        ])
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => {
            // User cancelled or zenity failed
            eprintln!(
                "Zenity exited with status: {}. Proceeding without sudo password.",
                output.status
            );
            String::new()
        }
        Err(e) => {
            eprintln!("Failed to launch zenity for password prompt: {}. Proceeding without sudo password.", e);
            String::new()
        }
    }
}

/// Run a command that requires sudo privileges.
/// Pipes the password to `sudo -S` via stdin so the user is not prompted interactively.
/// The `command` and `args` represent the full command to run under sudo
/// (e.g., command="iw", args=["dev", "wlan0", "interface", "add", "mon0", "type", "monitor"]).
pub async fn run_sudo_command(
    command: String,
    args: Vec<String>,
    password: String,
) -> Result<Output, Arc<std::io::Error>> {
    use tokio::io::AsyncWriteExt;

    let mut cmd = tokio::process::Command::new("sudo");
    cmd.arg("-S").arg(command).args(args);
    cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn().map_err(Arc::new)?;

    // Write password to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(format!("{}\n", password).as_bytes()).await.map_err(Arc::new)?;
        stdin.flush().await.map_err(Arc::new)?;
    }

    // Wait for output
    child.wait_with_output().await.map_err(Arc::new)
}

pub async fn run_command(command: String, args: Vec<String>) -> Result<Output, Arc<std::io::Error>> {
    tokio::process::Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(Arc::new)
}

fn in_path(bin: &str) -> bool {
    if bin.contains('/') {
        return Path::new(bin).is_file();
    }
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| dir.join(bin).is_file())
        })
        .unwrap_or(false)
}

/// Resolve terminal binary: user pref → $TERMINAL → common fallbacks.
pub fn resolve_terminal(pref: &str) -> Result<String, String> {
    let pref = pref.trim();
    if !pref.is_empty() {
        let bin = pref.split_whitespace().next().unwrap_or(pref);
        if in_path(bin) {
            return Ok(pref.to_string());
        }
        return Err(format!("Terminal not found in PATH: {bin}"));
    }

    if let Ok(term) = std::env::var("TERMINAL") {
        let t = term.trim();
        if !t.is_empty() {
            let bin = t.split_whitespace().next().unwrap_or(t);
            if in_path(bin) {
                return Ok(t.to_string());
            }
        }
    }

    const CANDIDATES: &[&str] = &[
        "kitty",
        "ghostty",
        "alacritty",
        "wezterm",
        "foot",
        "gnome-terminal",
        "kgx",
        "konsole",
        "xfce4-terminal",
        "mate-terminal",
        "xterm",
        "x-terminal-emulator",
    ];
    for c in CANDIDATES {
        if in_path(c) {
            return Ok((*c).to_string());
        }
    }
    Err("No terminal emulator found. Set one in Settings (e.g. kitty, alacritty, xterm).".into())
}

fn terminal_argv(term_pref: &str, script: &str) -> Result<(String, Vec<String>), String> {
    let resolved = resolve_terminal(term_pref)?;
    let mut parts = resolved.split_whitespace();
    let program = parts.next().unwrap_or("xterm").to_string();
    let extra: Vec<String> = parts.map(|s| s.to_string()).collect();

    let basename = Path::new(&program)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(program.as_str());

    let mut args = extra;
    match basename {
        "gnome-terminal" | "kgx" => {
            args.extend([
                "--".into(),
                "bash".into(),
                "-c".into(),
                script.to_string(),
            ]);
        }
        "xfce4-terminal" | "mate-terminal" => {
            args.extend([
                "-x".into(),
                "bash".into(),
                "-c".into(),
                script.to_string(),
            ]);
        }
        "wezterm" => {
            args.extend([
                "start".into(),
                "--".into(),
                "bash".into(),
                "-c".into(),
                script.to_string(),
            ]);
        }
        "konsole" => {
            args.extend([
                "-e".into(),
                "bash".into(),
                "-c".into(),
                script.to_string(),
            ]);
        }
        // kitty, alacritty, foot, xterm, x-terminal-emulator, ghostty, …
        _ => {
            args.extend([
                "-e".into(),
                "bash".into(),
                "-c".into(),
                script.to_string(),
            ]);
        }
    }
    Ok((program, args))
}

pub struct SpawnedTerminal {
    pub pid: u32,
    pub pgid: i32,
    pub child: tokio::process::Child,
    pub terminal: String,
}

/// Spawn a terminal running `script` in a new process group (for reliable kill).
pub fn spawn_terminal_job(term_pref: &str, script: &str) -> Result<SpawnedTerminal, String> {
    let (program, args) = terminal_argv(term_pref, script)?;
    let display = format!("{program} {}", args.iter().take(3).cloned().collect::<Vec<_>>().join(" "));

    let mut cmd = tokio::process::Command::new(&program);
    cmd.args(&args);
    // Detach from our stdio; terminal owns the session.
    cmd.stdin(std::process::Stdio::null());
    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());

    #[cfg(unix)]
    {
        #[allow(unused_imports)]
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }

    let child = cmd
        .spawn()
        .map_err(|e| format!("Failed to spawn terminal '{program}': {e}"))?;
    let pid = child
        .id()
        .ok_or_else(|| format!("Terminal '{program}' spawned without a PID"))?;

    Ok(SpawnedTerminal {
        pid,
        pgid: pid as i32,
        child,
        terminal: display,
    })
}

pub async fn wait_child(mut child: tokio::process::Child) -> Result<Option<i32>, String> {
    let status = child
        .wait()
        .await
        .map_err(|e| format!("wait failed: {e}"))?;
    Ok(status.code())
}

/// Terminate a process group (terminal + children like airodump).
pub fn kill_process_group(pgid: i32) {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        let group = Pid::from_raw(-pgid);
        let _ = kill(group, Signal::SIGTERM);
        std::thread::sleep(Duration::from_millis(250));
        let _ = kill(group, Signal::SIGKILL);
    }
    #[cfg(not(unix))]
    {
        let _ = pgid;
    }
}
