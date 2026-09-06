use std::process::Command;

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub active: bool,
    pub status: String,
}

pub fn list_services(filter: &str) -> Vec<Service> {
    let mut services = Vec::new();

    let common_services = vec![
        ("NetworkManager", "Менеджер сети"),
        ("bluetooth", "Bluetooth сервис"),
        ("cups", "Драйверы печати"),
        ("ssh", "SSH сервер"),
        ("docker", "Docker контейнеры"),
        ("pipewire", "Аудио сервер"),
        ("pipewire-pulse", "PulseAudio совместимость"),
        ("wireplumber", "Менеджер аудио устройств"),
        ("powertop", "Оптимизация батареи"),
        ("tlp", "Управление питанием"),
        ("fstrim.timer", "Автоматический TRIM"),
        ("firewalld", "Межсетевой экран"),
        ("chronyd", "Синхронизация времени"),
        ("systemd-timesyncd", "Синхронизация времени (systemd)"),
        ("gpu-manager", "GPU менеджер"),
        ("multipathd", "Multipath для дисков"),
        ("ModemManager", "Мобильные модемы"),
    ];

    for (name, desc) in common_services {
        if !filter.is_empty() && !name.to_lowercase().contains(&filter.to_lowercase()) {
            continue;
        }

        let enabled = is_service_enabled(name);
        let active = is_service_active(name);
        let status = if active {
            "active".to_string()
        } else if enabled {
            "enabled".to_string()
        } else {
            "inactive".to_string()
        };

        services.push(Service {
            name: name.to_string(),
            description: desc.to_string(),
            enabled,
            active,
            status,
        });
    }

    services
}

fn is_service_enabled(name: &str) -> bool {
    let output = match Command::new("systemctl")
        .args(["is-enabled", name])
        .output()
    {
        Ok(o) => o,
        _ => return false,
    };

    String::from_utf8_lossy(&output.stdout).trim() == "enabled"
}

fn is_service_active(name: &str) -> bool {
    let output = match Command::new("systemctl")
        .args(["is-active", name])
        .output()
    {
        Ok(o) => o,
        _ => return false,
    };

    String::from_utf8_lossy(&output.stdout).trim() == "active"
}

pub fn enable_service(name: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["systemctl", "enable", name])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn disable_service(name: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["systemctl", "disable", name])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn start_service(name: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["systemctl", "start", name])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn stop_service(name: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["systemctl", "stop", name])
        .output()
        .map_err(|e| format!("Failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
