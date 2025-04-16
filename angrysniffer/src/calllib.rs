
use std::os::unix::thread;
use std::process::{Command, ExitCode, ExitStatus, Stdio};
use std::io::{BufRead, BufReader};
use std::string;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::fs::{self, File};
use std::path::Path;
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


pub fn kill_net_services() -> bool {
    let mut cmd = String::from("sudo airmon-ng check kill");
    let mut exit_status = run_cmd_captured(&mut cmd);
    println!("Network services have been stopped with exit status: {}", exit_status);
    return exit_status.success();
}

pub fn run_pipe_collect(cmd: String) -> String{
        let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");
    // Capture the output in real-time
    let mut output: String =String::new();
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(output_line) => {
                    println!("{}", output_line); // Print each line of output
                    output.push_str(&output_line);
                    output.push('\n'); // Add a newline to maintain output format
                }, 
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    // Wait for the command to finish
    let status = child.wait().expect("Failed to wait on child");
    println!("Command exited with status: {}", status);
    return output;

}

pub fn run_cmd_for_duration(cmd: String, duration: Duration) -> () {
    let start_time = Instant::now();

    while start_time.elapsed() < duration {
        println!("dbg4");
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn command");
        println!("dbg3");
        if let Some(stdout) = child.stdout.take() {
            println!("dbg5");
            let reader = BufReader::new(stdout);
            println!("dbg7");
            for line in reader.lines() {
                println!("dbg6");
                match line {
                    Ok(output_line) => println!("{}", output_line), // Print each line of output
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
            }
        }
        println!("dbg2");
        let status = child.wait().expect("Failed to wait on child");
        println!("Command exited with status: {}", status);
        println!("dbg");
        // Optional: Add a small delay between iterations to avoid overwhelming the system
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Finished running the script for the specified duration.");
}


pub fn run_cmd_captured(cmd: &mut String) -> ExitStatus {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");
    // Capture the output in real-time
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(output) => println!("{}", output), // Print each line of output
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    }

    // Wait for the command to finish
    let status = child.wait().expect("Failed to wait on child");
    println!("Command exited with status: {}", status);
    return status;
}

pub fn ud_inf(inf: String,state: state) -> bool{
    let mut cmd = format!("sudo ip link set {} {}", inf, match state {
        state::Up => "up",
        state::Down => "down",
    });
    
    let excode: ExitStatus = run_cmd_captured(&mut cmd);
    
    return excode.success();
}

pub fn setUpMOnitor(inf: String,mon: String) -> bool {
    let mut cmd = format!("sudo iw dev {} interface add {} type monitor", inf, mon);
    let excode: ExitStatus = run_cmd_captured(&mut cmd);
    return excode.success();
}

pub fn tearDownMonitor(mon: String) -> bool {
    let mut cmd = format!("sudo iw dev {} del", mon);
    let excode: ExitStatus = run_cmd_captured(&mut cmd);
    return excode.success();
}

pub fn liftNetServices(inf: String, mon: String) -> bool{
    if !ud_inf(inf.clone(), state::Up) {
        eprintln!("Error: Failed to bring up interface {}", inf);
        return false;
    }
    
    if !ud_inf(mon.clone(), state::Down) {
        eprintln!("Error: Failed to bring down monitor interface {}", mon);
    }
    let mut cmd = format!("sudo systemctl restart NetworkManager.service;sudo systemctl restart wpa_supplicant.service");
    let excode: ExitStatus = run_cmd_captured(&mut cmd);
    return excode.success();
}

pub fn get_all_infs() -> Vec<String> {
    todo!();
}

pub fn scan_AP(mon: String, dur: Duration,path_o: String) -> bool {
    run_cmd_for_duration(format!("sudo airodump-ng {} --output-format csv -w {}",mon,path_o), dur);


    return false;
}

pub fn parseNetworkList(path_o: String) -> Vec<AP> {

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

