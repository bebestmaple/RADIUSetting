use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// 证书信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub fingerprint: String,
    pub issuer: String,
    pub subject: String,
    pub valid_from: String,
    pub valid_to: String,
    pub serial_number: String,
}

/// 信任状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStatus {
    pub fingerprint: String,
    pub trusted: bool,
    pub trusted_at: Option<String>,
    pub expires_at: Option<String>,
}

/// 信任数据库
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TrustDatabase {
    trusted_certs: Vec<TrustStatus>,
}

impl TrustDatabase {
    fn new() -> Self {
        Self {
            trusted_certs: Vec::new(),
        }
    }

    fn load() -> Result<Self> {
        let db_path = get_trust_db_path()?;

        if !db_path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&db_path)?;
        let db: TrustDatabase = serde_json::from_str(&content)?;
        Ok(db)
    }

    fn save(&self) -> Result<()> {
        let db_path = get_trust_db_path()?;

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&db_path, content)?;
        Ok(())
    }
}

/// 获取信任数据库路径
fn get_trust_db_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?;

    Ok(config_dir.join("radius-client").join("trusted_certs.json"))
}

/// 计算证书 SHA-256 指纹
pub fn calculate_fingerprint(cert_data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(cert_data);
    let result = hasher.finalize();

    result.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(":")
}

/// 解析证书信息（占位实现）
pub fn parse_certificate(cert_data: &[u8]) -> Result<CertificateInfo> {
    let fingerprint = calculate_fingerprint(cert_data);

    Ok(CertificateInfo {
        fingerprint: fingerprint.clone(),
        issuer: "CN=Example RADIUS CA".to_string(),
        subject: "CN=radius.example.com".to_string(),
        valid_from: "2025-01-01T00:00:00Z".to_string(),
        valid_to: "2027-12-31T23:59:59Z".to_string(),
        serial_number: "1234567890".to_string(),
    })
}

/// 检查证书是否已信任
pub fn is_certificate_trusted(fingerprint: &str) -> Result<bool> {
    let db = TrustDatabase::load()?;

    Ok(db.trusted_certs.iter().any(|ts|
        ts.fingerprint == fingerprint && ts.trusted
    ))
}

/// 信任证书
pub fn trust_certificate(cert_info: &CertificateInfo) -> Result<()> {
    let mut db = TrustDatabase::load()?;

    let existing = db.trusted_certs.iter_mut()
        .find(|ts| ts.fingerprint == cert_info.fingerprint);

    if let Some(ts) = existing {
        ts.trusted = true;
        ts.trusted_at = Some(chrono::Utc::now().to_rfc3339());
        ts.expires_at = Some(cert_info.valid_to.clone());
    } else {
        db.trusted_certs.push(TrustStatus {
            fingerprint: cert_info.fingerprint.clone(),
            trusted: true,
            trusted_at: Some(chrono::Utc::now().to_rfc3339()),
            expires_at: Some(cert_info.valid_to.clone()),
        });
    }

    db.save()?;

    tracing::info!("证书已添加到信任列表: {}", cert_info.fingerprint);
    Ok(())
}

/// 撤销信任
pub fn revoke_trust(fingerprint: &str) -> Result<()> {
    let mut db = TrustDatabase::load()?;

    if let Some(ts) = db.trusted_certs.iter_mut().find(|ts| ts.fingerprint == fingerprint) {
        ts.trusted = false;
        db.save()?;
        tracing::info!("证书信任已撤销: {}", fingerprint);
        Ok(())
    } else {
        bail!("证书不在信任列表中: {}", fingerprint);
    }
}

/// 列出所有已信任证书
pub fn list_trusted_certificates() -> Result<Vec<TrustStatus>> {
    let db = TrustDatabase::load()?;
    Ok(db.trusted_certs.iter()
        .filter(|ts| ts.trusted)
        .cloned()
        .collect())
}

/// 检查证书是否过期
pub fn is_certificate_expired(cert_info: &CertificateInfo) -> Result<bool> {
    use chrono::{DateTime, Utc};

    let valid_to = DateTime::parse_from_rfc3339(&cert_info.valid_to)?;
    let now = Utc::now();

    Ok(valid_to < now)
}

/// 获取证书过期天数
pub fn days_until_expiry(cert_info: &CertificateInfo) -> Result<i64> {
    use chrono::{DateTime, Utc};

    let valid_to = DateTime::parse_from_rfc3339(&cert_info.valid_to)?;
    let now = Utc::now();

    let duration = valid_to.signed_duration_since(now);
    Ok(duration.num_days())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_calculation() {
        let data = b"test certificate data";
        let fp = calculate_fingerprint(data);

        // 验证格式：32 个十六进制字节，用冒号分隔
        assert!(fp.contains(':'));
        assert_eq!(fp.split(':').count(), 32);

        // 验证每段都是 2 个十六进制字符
        for segment in fp.split(':') {
            assert_eq!(segment.len(), 2);
            assert!(segment.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    #[test]
    fn test_fingerprint_consistency() {
        let data = b"test data";
        let fp1 = calculate_fingerprint(data);
        let fp2 = calculate_fingerprint(data);

        // 同样的数据应生成相同的指纹
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn test_fingerprint_different_data() {
        let data1 = b"test data 1";
        let data2 = b"test data 2";
        let fp1 = calculate_fingerprint(data1);
        let fp2 = calculate_fingerprint(data2);

        // 不同的数据应生成不同的指纹
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn test_trust_workflow() {
        let cert_info = CertificateInfo {
            fingerprint: "AA:BB:CC:DD:EE:FF:00:11:22:33:44:55:66:77:88:99:AA:BB:CC:DD:EE:FF:00:11:22:33:44:55:66:77:88:99".to_string(),
            issuer: "CN=Test CA".to_string(),
            subject: "CN=test.com".to_string(),
            valid_from: "2025-01-01T00:00:00Z".to_string(),
            valid_to: "2027-12-31T23:59:59Z".to_string(),
            serial_number: "123456".to_string(),
        };

        // 信任证书
        trust_certificate(&cert_info).expect("信任证书应成功");

        // 现在已信任
        let is_trusted = is_certificate_trusted(&cert_info.fingerprint).unwrap_or(false);
        assert!(is_trusted, "证书应已被信任");

        // 撤销信任
        revoke_trust(&cert_info.fingerprint).expect("撤销信任应成功");

        // 现在未信任
        let is_trusted = is_certificate_trusted(&cert_info.fingerprint).unwrap_or(true);
        assert!(!is_trusted, "证书信任应已撤销");
    }

    #[test]
    fn test_certificate_expiry() {
        let expired_cert = CertificateInfo {
            fingerprint: "FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF:FF".to_string(),
            issuer: "CN=Expired CA".to_string(),
            subject: "CN=expired.com".to_string(),
            valid_from: "2020-01-01T00:00:00Z".to_string(),
            valid_to: "2021-12-31T23:59:59Z".to_string(),
            serial_number: "999".to_string(),
        };

        let is_expired = is_certificate_expired(&expired_cert).unwrap_or(false);
        assert!(is_expired, "2021年过期的证书应被检测为已过期");

        let days = days_until_expiry(&expired_cert).unwrap_or(0);
        assert!(days < 0, "过期证书的剩余天数应为负数");
    }

    #[test]
    fn test_certificate_not_expired() {
        let future_cert = CertificateInfo {
            fingerprint: "00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00:00".to_string(),
            issuer: "CN=Future CA".to_string(),
            subject: "CN=future.com".to_string(),
            valid_from: "2025-01-01T00:00:00Z".to_string(),
            valid_to: "2030-12-31T23:59:59Z".to_string(),
            serial_number: "888".to_string(),
        };

        let is_expired = is_certificate_expired(&future_cert).unwrap_or(true);
        assert!(!is_expired, "2030年过期的证书不应被检测为已过期");

        let days = days_until_expiry(&future_cert).unwrap_or(-1);
        assert!(days > 0, "未过期证书的剩余天数应为正数");
    }

    #[test]
    fn test_parse_certificate_basic() {
        let cert_data = b"fake cert data";
        let cert_info = parse_certificate(cert_data).expect("解析应成功");

        // 验证返回的结构体字段非空
        assert!(!cert_info.fingerprint.is_empty());
        assert!(!cert_info.issuer.is_empty());
        assert!(!cert_info.subject.is_empty());
    }
}
