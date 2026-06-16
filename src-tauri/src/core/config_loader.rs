use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub version: String,
    pub authentication: AuthConfig,
    pub logging: LogConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub protocol: String, // "PEAP"
    pub inner_method: String, // "MSCHAPv2"
    pub anonymous_identity: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogConfig {
    pub level: String,
    pub file_path: PathBuf,
    pub max_size_mb: u64,
}

pub fn load_config() -> Result<AppConfig> {
    let config_path = PathBuf::from("config.json");
    let content = std::fs::read_to_string(config_path)?;
    let config: AppConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn default_config() -> AppConfig {
    AppConfig {
        version: "0.1.0".to_string(),
        authentication: AuthConfig {
            protocol: "PEAP".to_string(),
            inner_method: "MSCHAPv2".to_string(),
            anonymous_identity: "anonymous".to_string(),
        },
        logging: LogConfig {
            level: "info".to_string(),
            file_path: PathBuf::from("./logs/radius-client.log"),
            max_size_mb: 10,
        },
    }
}
