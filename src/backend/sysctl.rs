use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SysctlParam {
    pub key: String,
    pub value: String,
    pub description: String,
    pub category: String,
    pub default: String,
}

pub fn read_sysctl(key: &str) -> Result<String, String> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg(key)
        .output()
        .map_err(|e| format!("Failed to execute sysctl: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn write_sysctl(key: &str, value: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg("sysctl")
        .arg("-w")
        .arg(format!("{}={}", key, value))
        .output()
        .map_err(|e| format!("Failed to execute sysctl: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn get_common_params() -> Vec<SysctlParam> {
    let mut params = vec![
        SysctlParam {
            key: "vm.swappiness".to_string(),
            value: String::new(),
            description: "Склонность системы к использованию swap (0-200)".to_string(),
            category: "Memory".to_string(),
            default: "60".to_string(),
        },
        SysctlParam {
            key: "vm.dirty_ratio".to_string(),
            value: String::new(),
            description: "Процент ОЗУ для грязных страниц перед записью на диск".to_string(),
            category: "Memory".to_string(),
            default: "20".to_string(),
        },
        SysctlParam {
            key: "vm.dirty_background_ratio".to_string(),
            value: String::new(),
            description: "Процент ОЗУ для фоновой записи".to_string(),
            category: "Memory".to_string(),
            default: "10".to_string(),
        },
        SysctlParam {
            key: "vm.vfs_cache_pressure".to_string(),
            value: String::new(),
            description: "Давление на кэш файловой системы".to_string(),
            category: "Memory".to_string(),
            default: "100".to_string(),
        },
        SysctlParam {
            key: "net.core.rmem_max".to_string(),
            value: String::new(),
            description: "Максимальный размер буфера приема".to_string(),
            category: "Network".to_string(),
            default: "212992".to_string(),
        },
        SysctlParam {
            key: "net.core.wmem_max".to_string(),
            value: String::new(),
            description: "Максимальный размер буфера отправки".to_string(),
            category: "Network".to_string(),
            default: "212992".to_string(),
        },
        SysctlParam {
            key: "net.ipv4.tcp_fastopen".to_string(),
            value: String::new(),
            description: "Ускоренное соединение TCP".to_string(),
            category: "Network".to_string(),
            default: "1".to_string(),
        },
        SysctlParam {
            key: "net.ipv4.tcp_mtu_probing".to_string(),
            value: String::new(),
            description: "Автоматическое определение MTU".to_string(),
            category: "Network".to_string(),
            default: "0".to_string(),
        },
        SysctlParam {
            key: "kernel.sched_autogroup_enabled".to_string(),
            value: String::new(),
            description: "Автогруппировка процессов (для десктопа)".to_string(),
            category: "Scheduler".to_string(),
            default: "1".to_string(),
        },
        SysctlParam {
            key: "kernel.nmi_watchdog".to_string(),
            value: String::new(),
            description: "Watchdog для NMI (отключить для виртуалок)".to_string(),
            category: "Scheduler".to_string(),
            default: "1".to_string(),
        },
    ];

    for param in &mut params {
        if let Ok(val) = read_sysctl(&param.key) {
            param.value = val;
        } else {
            param.value = param.default.clone();
        }
    }

    params
}

pub fn get_all_params() -> HashMap<String, String> {
    let output = match Command::new("sysctl").arg("-a").output() {
        Ok(o) => o,
        _ => return HashMap::new(),
    };

    let mut map = HashMap::new();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some((key, value)) = line.split_once(" = ") {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    map
}
