use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SwapInfo {
    pub total: String,
    pub used: String,
    pub free: String,
    pub swappiness: String,
}

#[derive(Debug, Clone)]
pub struct ZramInfo {
    pub available: bool,
    pub size: String,
    pub algorithm: String,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub device: String,
    pub mount: String,
    pub fs_type: String,
    pub size: String,
    pub is_ssd: bool,
    pub scheduler: String,
}

pub fn get_swap_info() -> SwapInfo {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();

    let mut total = String::from("0 kB");
    let mut free = String::from("0 kB");

    for line in meminfo.lines() {
        if line.starts_with("SwapTotal:") {
            total = line.split_whitespace().nth(1).unwrap_or("0").to_string();
        }
        if line.starts_with("SwapFree:") {
            free = line.split_whitespace().nth(1).unwrap_or("0").to_string();
        }
    }

    let total_kb: u64 = total.parse().unwrap_or(0);
    let free_kb: u64 = free.parse().unwrap_or(0);
    let used_kb = total_kb.saturating_sub(free_kb);

    let swappiness = fs::read_to_string("/proc/sys/vm/swappiness")
        .unwrap_or_default()
        .trim()
        .to_string();

    SwapInfo {
        total: format!("{} MB", total_kb / 1024),
        used: format!("{} MB", used_kb / 1024),
        free: format!("{} MB", free_kb / 1024),
        swappiness,
    }
}

pub fn get_zram_info() -> ZramInfo {
    let output = match Command::new("zramctl").output() {
        Ok(o) => o,
        _ => {
            return ZramInfo {
                available: false,
                size: "N/A".to_string(),
                algorithm: "N/A".to_string(),
            }
        }
    };

    if !output.status.success() {
        return ZramInfo {
            available: false,
            size: "N/A".to_string(),
            algorithm: "N/A".to_string(),
        };
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() > 1 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() >= 4 {
            return ZramInfo {
                available: true,
                size: parts[2].to_string(),
                algorithm: parts[3].to_string(),
            };
        }
    }

    ZramInfo {
        available: false,
        size: "N/A".to_string(),
        algorithm: "N/A".to_string(),
    }
}

pub fn get_disks() -> Vec<DiskInfo> {
    let mut disks = Vec::new();

    let output = match Command::new("lsblk")
        .args(["-o", "NAME,MOUNTPOINT,FSTYPE,SIZE,ROTA,_SCHED"])
        .output()
    {
        Ok(o) => o,
        _ => return disks,
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let is_ssd = parts[4] == "0"; // ROTA=0 means SSD
                disks.push(DiskInfo {
                    device: format!("/dev/{}", parts[0]),
                    mount: parts[1].to_string(),
                    fs_type: parts[2].to_string(),
                    size: parts[3].to_string(),
                    is_ssd,
                    scheduler: if parts.len() > 5 { parts[5].to_string() } else { String::new() },
                });
            }
        }
    }

    disks
}

pub fn get_io_scheduler(device: &str) -> String {
    let path = format!(
        "/sys/block/{}/queue/scheduler",
        device.trim_start_matches("/dev/")
    );
    fs::read_to_string(&path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn set_io_scheduler(device: &str, scheduler: &str) -> Result<(), String> {
    let path = format!(
        "/sys/block/{}/queue/scheduler",
        device.trim_start_matches("/dev/")
    );

    Command::new("sudo")
        .args(["sh", "-c", &format!("echo {} > {}", scheduler, path)])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    Ok(())
}

pub fn set_swappiness(value: u32) -> Result<(), String> {
    Command::new("sudo")
        .args(["sysctl", "-w", &format!("vm.swappiness={}", value)])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    Ok(())
}

pub fn get_trim_status() -> bool {
    let output = match Command::new("systemctl")
        .args(["is-active", "fstrim.timer"])
        .output()
    {
        Ok(o) => o,
        _ => return false,
    };

    String::from_utf8_lossy(&output.stdout).trim() == "active"
}

pub fn enable_trim() -> Result<(), String> {
    Command::new("sudo")
        .args(["systemctl", "enable", "--now", "fstrim.timer"])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    Ok(())
}
