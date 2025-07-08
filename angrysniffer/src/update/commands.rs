
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

pub async fn run_command(command: String, args: Vec<String>) -> Result<Output, Arc<std::io::Error>> {
    tokio::process::Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(Arc::new)
}
