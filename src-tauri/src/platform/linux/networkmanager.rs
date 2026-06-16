use anyhow::{Result, bail};
use std::process::Command;

pub fn configure_via_nm(interface: &str, username: &str, password: &str) -> Result<()> {
    let conn_name = format!("RADIUS-{}", interface);

    Command::new("nmcli")
        .args(&["con", "delete", &conn_name])
        .output()
        .ok();

    let output = Command::new("nmcli")
        .args(&[
            "connection", "add",
            "type", "ethernet",
            "con-name", &conn_name,
            "ifname", interface,
            "802-1x.eap", "peap",
            "802-1x.phase2-auth", "mschapv2",
            "802-1x.identity", username,
            "802-1x.password", password,
            "802-1x.anonymous-identity", "anonymous",
            "connection.autoconnect", "yes",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("nmcli 创建连接失败: {}", stderr);
    }

    let output = Command::new("nmcli")
        .args(&["connection", "up", &conn_name])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("nmcli 激活连接失败: {}", stderr);
    }

    Ok(())
}

pub fn configure_via_wpa_supplicant(interface: &str, username: &str, password: &str) -> Result<()> {
    let config = format!(
        r#"network={{
    key_mgmt=IEEE8021X
    eap=PEAP
    phase2="auth=MSCHAPV2"
    identity="{}"
    password="{}"
    anonymous_identity="anonymous"
    eapol_flags=0
}}
"#,
        username, password
    );

    let config_path = "/etc/wpa_supplicant/wpa_supplicant_radius.conf";
    std::fs::write(config_path, config)?;

    let output = Command::new("wpa_supplicant")
        .args(&[
            "-B",
            "-i", interface,
            "-c", config_path,
            "-D", "wired",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("wpa_supplicant 启动失败: {}", stderr);
    }

    Ok(())
}

pub fn disable_via_nm(interface: &str) -> Result<()> {
    let conn_name = format!("RADIUS-{}", interface);

    let output = Command::new("nmcli")
        .args(&["connection", "down", &conn_name])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("nmcli 断开连接失败: {}", stderr);
    }

    Ok(())
}

pub fn disable_via_wpa_supplicant(_interface: &str) -> Result<()> {
    Command::new("pkill")
        .args(&["wpa_supplicant"])
        .output()?;

    Ok(())
}

pub fn get_connection_status(interface: &str) -> Result<String> {
    let output = Command::new("nmcli")
        .args(&["-t", "-f", "GENERAL.STATE", "device", "show", interface])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

pub fn get_ip_address(interface: &str) -> Result<String> {
    let output = Command::new("ip")
        .args(&["-4", "addr", "show", interface])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("inet ") {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 {
                return Ok(parts[1].to_string());
            }
        }
    }

    Ok("N/A".to_string())
}

pub fn check_service_running(service_name: &str) -> Result<bool> {
    let output = Command::new("systemctl")
        .args(&["is-active", service_name])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim() == "active")
}

pub fn check_package_installed(package: &str) -> Result<bool> {
    let dpkg = Command::new("dpkg")
        .args(&["-s", package])
        .output();

    if let Ok(output) = dpkg {
        if output.status.success() {
            return Ok(true);
        }
    }

    let rpm = Command::new("rpm")
        .args(&["-q", package])
        .output();

    if let Ok(output) = rpm {
        if output.status.success() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn list_interfaces() -> Result<Vec<String>> {
    let output = Command::new("ip")
        .args(&["link", "show"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut interfaces = vec![];

    for line in stdout.lines() {
        if line.contains(":") && !line.starts_with(" ") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let iface = parts[1].trim();
                if iface != "lo" {
                    interfaces.push(iface.to_string());
                }
            }
        }
    }

    Ok(interfaces)
}
