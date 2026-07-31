use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct AP {
    pub bssid: String,
    pub first_seen: String,
    pub last_seen: String,
    pub channel: u8,
    pub speed: String,
    pub privacy: String,
    pub cipher: String,
    pub authentication: String,
    pub power: i32,
    pub beacons: u32,
    pub iv: u32,
    pub lan_ip: String,
    pub id_length: u32,
    pub essid: String,
    pub key: String,
}

impl AP {
    pub fn empty() -> AP {
        AP {
            bssid: String::new(),
            first_seen: String::new(),
            last_seen: String::new(),
            channel: 0,
            speed: String::new(),
            privacy: String::new(),
            cipher: String::new(),
            authentication: String::new(),
            power: 0,
            beacons: 0,
            iv: 0,
            lan_ip: String::new(),
            id_length: 0,
            essid: String::new(),
            key: String::new(),
        }
    }

    pub fn new(
        bssid: String,
        first_seen: String,
        last_seen: String,
        channel: u8,
        speed: String,
        privacy: String,
        cipher: String,
        authentication: String,
        power: i32,
        beacons: u32,
        iv: u32,
        lan_ip: String,
        id_length: u32,
        essid: String,
        key: String,
    ) -> AP {
        AP {
            bssid,
            first_seen,
            last_seen,
            channel,
            speed,
            privacy,
            cipher,
            authentication,
            power,
            beacons,
            iv,
            lan_ip,
            id_length,
            essid,
            key,
        }
    }

    pub fn from_string(buffer: &str) -> Option<AP> {
        let parts: Vec<&str> = buffer.split(',').map(|s| s.trim()).collect();
        if parts.len() < 14 {
            return None;
        }
        Some(AP::new(
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
            parts[3].parse().unwrap_or(0),
            parts[4].to_string(),
            parts[5].to_string(),
            parts[6].to_string(),
            parts[7].to_string(),
            parts[8].parse().unwrap_or(-1024),
            parts[9].parse().unwrap_or(0),
            parts[10].parse().unwrap_or(0),
            parts[11].to_string(),
            parts[12].parse().unwrap_or(0),
            parts.get(13).unwrap_or(&"").to_string(),
            parts.get(14).unwrap_or(&"").to_string(),
        ))
    }

    pub fn summary(&self) -> String {
        let essid = if self.essid.is_empty() {
            "<hidden>"
        } else {
            &self.essid
        };
        format!(
            "{}  {:>4} dBm  ch {:>2}  {}  [{}]",
            self.bssid, self.power, self.channel, essid, self.privacy
        )
    }

    pub fn has_target(&self) -> bool {
        !self.bssid.is_empty()
    }
}

pub fn parse_network_list(path: &str) -> Result<Vec<AP>, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open '{path}': {e}"))?;
    let reader = BufReader::new(file);
    let mut ap_list = Vec::new();

    for line in reader.lines() {
        let record = match line {
            Ok(r) => r,
            Err(e) => return Err(format!("Error reading '{path}': {e}")),
        };

        if record.starts_with("Station") {
            break;
        }
        if record.starts_with("BSSID") {
            continue;
        }
        if record.chars().filter(|&c| c == ',').count() < 13 {
            continue;
        }

        if let Some(ap) = AP::from_string(&record) {
            if ap.bssid.len() >= 11 {
                ap_list.push(ap);
            }
        }
    }

    Ok(ap_list)
}
