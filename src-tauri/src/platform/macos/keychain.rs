use anyhow::{Result, bail};
use std::process::Command;

/// 导入证书到系统 Keychain
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

    tracing::info!("证书已导入到 macOS Keychain");
    Ok(())
}

/// 从 Keychain 删除证书
pub fn remove_certificate(cert_name: &str) -> Result<()> {
    let output = Command::new("security")
        .args(&[
            "delete-certificate",
            "-c", cert_name,
            "/Library/Keychains/System.keychain",
        ])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("证书删除失败: {}", stderr);
    }

    tracing::info!("证书已从 macOS Keychain 删除");
    Ok(())
}

/// 检查证书是否存在
pub fn certificate_exists(cert_name: &str) -> Result<bool> {
    let output = Command::new("security")
        .args(&[
            "find-certificate",
            "-c", cert_name,
            "/Library/Keychains/System.keychain",
        ])
        .output()?;

    Ok(output.status.success())
}

/// 验证证书
pub fn verify_certificate(cert_path: &str) -> Result<()> {
    let output = Command::new("security")
        .args(&["verify-cert", "-c", cert_path])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("证书验证失败: {}", stderr);
    }

    Ok(())
}
