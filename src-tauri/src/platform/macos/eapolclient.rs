use anyhow::{Result, bail};
use std::process::Command;

/// 配置 PEAP 802.1X 认证
pub fn configure_peap(interface: &str, username: &str, password: &str) -> Result<()> {
    let output = Command::new("networksetup")
        .args(&[
            "-setEAPOLClientUser",
            interface,
            username,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("设置 EAP 用户失败: {}", stderr);
    }

    let output = Command::new("networksetup")
        .args(&[
            "-setEAPOLClientPassword",
            interface,
            password,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("设置 EAP 密码失败: {}", stderr);
    }

    Ok(())
}

/// 禁用 802.1X 认证
pub fn disable_802_1x(interface: &str) -> Result<()> {
    let output = Command::new("networksetup")
        .args(&["-setEAPOLClientConfiguration", interface, "None"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("禁用 802.1X 失败: {}", stderr);
    }

    Ok(())
}

/// 获取接口状态
pub fn get_interface_status(interface: &str) -> Result<String> {
    let output = Command::new("networksetup")
        .args(&["-getEAPOLClientConfiguration", interface])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

/// 检查 eapolclient 是否运行
pub fn check_eapolclient_running() -> Result<bool> {
    let output = Command::new("pgrep")
        .args(&["-x", "eapolclient"])
        .output()?;

    Ok(output.status.success())
}

/// 列出可用网络接口
pub fn list_interfaces() -> Result<Vec<String>> {
    let output = Command::new("networksetup")
        .args(&["-listallhardwareports"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut interfaces = vec![];

    for line in stdout.lines() {
        if line.contains("Device:") {
            if let Some(iface) = line.split("Device:").nth(1) {
                interfaces.push(iface.trim().to_string());
            }
        }
    }

    Ok(interfaces)
}

/// 导入证书到 Keychain
pub fn import_certificate(cert_path: &str) -> Result<()> {
    let output = Command::new("security")
        .args(&[
            "add-trusted-cert",
            "-d",
            "-r", "trustRoot",
            "-k", "/Library/Keychains/System.keychain",
            cert_path,
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("证书导入失败: {}", stderr);
    }

    Ok(())
}
