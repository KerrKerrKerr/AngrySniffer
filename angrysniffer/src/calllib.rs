use std::io::{BufRead, BufReader};
use std::fs::File;
enum state {
    Up,
    Down
}

//BSSID	 First time seen	 Last time seen	 channel	 Speed	 Privacy	 Cipher	 Authentication	 Power	 # beacons	 # IV	 LAN IP	 ID-length	 ESSID	 Key
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
    pub fn empty() -> AP{
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

    pub fn new(bssid: String, first_seen: String, last_seen: String, channel: u8, speed: String, privacy: String, cipher: String, authentication: String, power: i32, beacons: u32, iv: u32, lan_ip: String, id_length: u32, essid: String, key: String) -> AP {
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
    //how it will be stored
    //BSSID, First time seen, Last time seen, channel, Speed, Privacy, Cipher, Authentication, Power, # beacons, # IV, LAN IP, ID-length, ESSID, Key
    //8:0D:17:F1:2C:F1, 2025-04-13 17:14:27, 2025-04-13 17:17:15, 11, 270, WPA2, CCMP, PSK, -71,        2,        0,   0.  0.  0.  0,   9, Tomasenok, 
    //3C:84:6A:C1:D9:21, 2025-04-13 17:14:26, 2025-04-13 17:17:14,  3, 540, WPA2, CCMP, PSK, -80,        3,        0,   0.  0.  0.  0,   5, Zahar, 
    //
    pub fn from_string(buffer: String) -> AP {
        let parts: Vec<&str> = buffer.split(',').map(|s| s.trim()).collect();
        if parts.len() != 15 {
            panic!("Invalid input string for AP");
        }
        AP::new(
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
            parts[13].to_string(),
            parts[14].to_string(),
        )
    }
    pub fn to_string_less(&mut self) -> String{
        
        format!("{}   {}   {}   {}", self.bssid,self.power,self.channel, self.essid)
    }

    pub fn to_string(&mut self) -> String{
        
        format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", 
            self.bssid,
            self.first_seen,
            self.last_seen,
            self.channel,
            self.speed,
            self.privacy,
            self.cipher,
            self.authentication,
            self.power,
            self.beacons,
            self.iv,
            self.lan_ip,
            self.id_length,
            self.essid,
            self.key
        )
    }
}




pub fn parse_network_list(path_o: String) -> Vec<AP> {

    let mut ap_list = Vec::new();


    let file = File::open(path_o).expect("Failed to open the file");
    let reader = BufReader::new(file);


    for line in reader.lines() {
        match line {
            Ok(record) => {
                if record.chars().filter(|&c| c == ',').count() != 14  || record.starts_with("BSSID"){
                    continue;
                }
                    
                if record.starts_with("Station") {
                    break;
                }
                let ap: AP = AP::from_string(record.clone());
                if !ap.essid.is_empty() {

                    ap_list.push(ap);
                }
            },
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    return ap_list;

}

