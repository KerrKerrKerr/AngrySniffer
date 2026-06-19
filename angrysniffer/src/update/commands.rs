
use std::process::Output;
use std::sync::Arc;

pub fn get_interface_names() -> Vec<String> {
    let interfaces = match std::fs::read_dir("/sys/class/net") {
        Ok(entries) => {
            let mut ifaces = vec!["none".to_string()];
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        ifaces.push(name.to_string());
                    }
                }
            }
            ifaces
        }
        Err(_) => vec!["failed to parse /sys/class/net".to_string()],
    };
    interfaces
}

pub fn get_monitor_interfaces() -> Vec<String> {
    let interfaces = match std::fs::read_dir("/sys/class/net") {
        Ok(entries) => {
            let mut monitor_ifaces = vec!["none".to_string()];
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.contains("mon") || is_monitor_interface(name) {
                            monitor_ifaces.push(name.to_string());
                        }
                    }
                }
            }
            monitor_ifaces
        }
        Err(_) => vec!["failed to parse /sys/class/net".to_string()],
    };
    interfaces
}

pub fn is_monitor_interface(interface_name: &str) -> bool {
    let type_path = format!("/sys/class/net/{}/type", interface_name);
    if let Ok(content) = std::fs::read_to_string(&type_path) {
        if let Ok(type_num) = content.trim().parse::<u32>() {
            return type_num == 803;
        }
    }
    false
}

pub fn neutrlize(strng: String) -> String {
    strng
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
        .collect()
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
