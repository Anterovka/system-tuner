use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub governor: String,
    pub available_governors: Vec<String>,
    pub frequency: String,
}

#[derive(Debug, Clone)]
pub struct PowerProfile {
    pub name: String,
    pub description: String,
    pub settings: Vec<PowerSetting>,
}

#[derive(Debug, Clone)]
pub struct PowerSetting {
    pub key: String,
    pub value: String,
    pub description: String,
}

pub fn get_cpu_info() -> CpuInfo {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let mut model = String::new();
    let mut cores = 0;

    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            model = line.split(':').nth(1).unwrap_or("").trim().to_string();
        }
        if line.starts_with("processor") {
            cores += 1;
        }
    }

    let governor = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
        .unwrap_or_default()
        .trim()
        .to_string();

    let available_governors = fs::read_to_string(
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors",
    )
    .unwrap_or_default()
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();

    let frequency = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        .unwrap_or_default()
        .trim()
        .parse::<u64>()
        .map(|f| format!("{} MHz", f / 1000))
        .unwrap_or_else(|_| "N/A".to_string());

    CpuInfo {
        model,
        cores,
        governor,
        available_governors,
        frequency,
    }
}

pub fn set_governor(governor: &str) -> Result<(), String> {
    Command::new("sudo")
        .args([
            "sh",
            "-c",
            &format!(
                "for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo {} > $cpu; done",
                governor
            ),
        ])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    Ok(())
}

pub fn get_power_profiles() -> Vec<PowerProfile> {
    vec![
        PowerProfile {
            name: "Performance".to_string(),
            description: "Максимальная производительность".to_string(),
            settings: vec![
                PowerSetting {
                    key: "CPU Governor".to_string(),
                    value: "performance".to_string(),
                    description: "Полная производительность CPU".to_string(),
                },
                PowerSetting {
                    key: "vm.swappiness".to_string(),
                    value: "10".to_string(),
                    description: "Меньше swap = больше RAM".to_string(),
                },
                PowerSetting {
                    key: "CPU Boost".to_string(),
                    value: "enabled".to_string(),
                    description: "Автоматическое разгонка CPU".to_string(),
                },
            ],
        },
        PowerProfile {
            name: "Balanced".to_string(),
            description: "Баланс между производительностью и батареей".to_string(),
            settings: vec![
                PowerSetting {
                    key: "CPU Governor".to_string(),
                    value: "schedutil".to_string(),
                    description: "Динамическое управление частотой".to_string(),
                },
                PowerSetting {
                    key: "vm.swappiness".to_string(),
                    value: "60".to_string(),
                    description: "Стандартное значение swap".to_string(),
                },
                PowerSetting {
                    key: "CPU Boost".to_string(),
                    value: "enabled".to_string(),
                    description: "Boost при необходимости".to_string(),
                },
            ],
        },
        PowerProfile {
            name: "Powersave".to_string(),
            description: "Максимальное время батареи".to_string(),
            settings: vec![
                PowerSetting {
                    key: "CPU Governor".to_string(),
                    value: "powersave".to_string(),
                    description: "Минимальное энергопотребление".to_string(),
                },
                PowerSetting {
                    key: "vm.swappiness".to_string(),
                    value: "80".to_string(),
                    description: "Больше swap для экономии RAM".to_string(),
                },
                PowerSetting {
                    key: "CPU Boost".to_string(),
                    value: "disabled".to_string(),
                    description: "Отключить автоматический разгон".to_string(),
                },
            ],
        },
    ]
}

pub fn apply_power_profile(profile: &PowerProfile) -> Result<(), String> {
    for setting in &profile.settings {
        match setting.key.as_str() {
            "CPU Governor" => set_governor(&setting.value)?,
            "vm.swappiness" => {
                Command::new("sudo")
                    .args(["sysctl", "-w", &format!("vm.swappiness={}", setting.value)])
                    .output()
                    .map_err(|e| format!("Failed: {}", e))?;
            }
            "CPU Boost" => {
                let path = "/sys/devices/system/cpu/cpboost";
                let value = if setting.value == "enabled" { "1" } else { "0" };
                Command::new("sudo")
                    .args(["sh", "-c", &format!("echo {} > {}", value, path)])
                    .output()
                    .map_err(|e| format!("Failed: {}", e))?;
            }
            _ => {}
        }
    }
    Ok(())
}
