use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// 获取证书存储目录
fn get_cert_dir() -> PathBuf {
    PathBuf::from("/etc/ssl/certs")
}

/// 导入 CA 证书
pub fn import_ca_certificate(cert_data: &[u8], cert_name: &str) -> Result<()> {
    let cert_dir = get_cert_dir();

    // 生成唯一文件名
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let filename = format!("radius-{}-{}.crt", cert_name, timestamp);
    let cert_path = cert_dir.join(&filename);

    // 写入证书文件
    fs::write(&cert_path, cert_data)?;

    tracing::info!("证书已复制到: {:?}", cert_path);

    // 更新证书存储（Debian/Ubuntu）
    let update_result = Command::new("update-ca-certificates")
        .output();

    if let Ok(output) = update_result {
        if output.status.success() {
            tracing::info!("证书存储已更新（update-ca-certificates）");
        }
    }

    // 尝试 RedHat/CentOS 的命令
    Command::new("update-ca-trust")
        .arg("extract")
        .output()
        .ok();

    Ok(())
}

/// 删除证书
pub fn remove_certificate(cert_name: &str) -> Result<()> {
    let cert_dir = get_cert_dir();

    // 查找匹配的证书文件
    let entries = fs::read_dir(&cert_dir)?;
    let mut removed = false;

    for entry in entries.flatten() {
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        if filename_str.contains(&format!("radius-{}", cert_name)) {
            fs::remove_file(entry.path())?;
            tracing::info!("已删除证书: {:?}", entry.path());
            removed = true;
        }
    }

    if removed {
        // 更新证书存储
        Command::new("update-ca-certificates")
            .output()
            .ok();
        Command::new("update-ca-trust")
            .arg("extract")
            .output()
            .ok();
    }

    Ok(())
}

/// 检查证书是否存在
pub fn certificate_exists(cert_name: &str) -> Result<bool> {
    let cert_dir = get_cert_dir();
    let entries = fs::read_dir(&cert_dir)?;

    for entry in entries.flatten() {
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        if filename_str.contains(&format!("radius-{}", cert_name)) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// 将证书导入到用户配置目录（无需 root）
pub fn import_user_certificate(cert_data: &[u8], cert_name: &str) -> Result<()> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?;

    let cert_dir = config_dir.join("radius-client").join("certs");
    fs::create_dir_all(&cert_dir)?;

    let cert_path = cert_dir.join(format!("{}.crt", cert_name));
    fs::write(&cert_path, cert_data)?;

    tracing::info!("证书已保存到用户目录: {:?}", cert_path);
    Ok(())
}
