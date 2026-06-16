use tauri::State;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use crate::core::auth_manager::{AuthManager, AuthState};
use crate::core::cert_validator::{CertificateInfo, TrustStatus};
use crate::platform::DiagResult;

/// 全局认证管理器状态
pub struct AppState {
    pub auth_manager: Mutex<AuthManager>,
}

/// 连接认证
#[tauri::command]
pub async fn connect_auth(
    interface: String,
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<AuthState, String> {
    let manager = state.auth_manager.lock().map_err(|e| e.to_string())?;

    manager.enable_auth(&interface, &username, &password)
        .map_err(|e| e.to_string())?;

    Ok(manager.get_state())
}

/// 断开认证
#[tauri::command]
pub async fn disconnect_auth(
    interface: String,
    state: State<'_, AppState>,
) -> Result<AuthState, String> {
    let manager = state.auth_manager.lock().map_err(|e| e.to_string())?;

    manager.disable_auth(&interface)
        .map_err(|e| e.to_string())?;

    Ok(manager.get_state())
}

/// 获取认证状态
#[tauri::command]
pub async fn get_auth_status(
    state: State<'_, AppState>,
) -> Result<AuthState, String> {
    let manager = state.auth_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_state())
}

/// 刷新状态
#[tauri::command]
pub async fn refresh_status(
    interface: String,
    state: State<'_, AppState>,
) -> Result<AuthState, String> {
    let manager = state.auth_manager.lock().map_err(|e| e.to_string())?;
    manager.refresh_status(&interface)
        .map_err(|e| e.to_string())
}

/// 信任证书
#[tauri::command]
pub async fn trust_certificate(
    cert_info: CertificateInfo,
    interface: String,
    username: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<AuthState, String> {
    let manager = state.auth_manager.lock().map_err(|e| e.to_string())?;

    manager.trust_and_continue(&cert_info, &interface, &username, &password)
        .map_err(|e| e.to_string())?;

    Ok(manager.get_state())
}

/// 获取已信任证书列表
#[tauri::command]
pub async fn list_trusted_certs() -> Result<Vec<TrustStatus>, String> {
    crate::core::cert_validator::list_trusted_certificates()
        .map_err(|e| e.to_string())
}

/// 撤销证书信任
#[tauri::command]
pub async fn revoke_certificate_trust(
    fingerprint: String,
) -> Result<(), String> {
    crate::core::cert_validator::revoke_trust(&fingerprint)
        .map_err(|e| e.to_string())
}

/// 系统诊断（需要 Serialize 支持）
#[derive(Serialize, Deserialize)]
pub struct SerializableDiagResult {
    pub services: Vec<SerializableServiceStatus>,
    pub overall_status: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableServiceStatus {
    pub name: String,
    pub running: bool,
    pub suggestion: Option<String>,
}

impl From<DiagResult> for SerializableDiagResult {
    fn from(result: DiagResult) -> Self {
        Self {
            services: result.services.into_iter().map(|s| SerializableServiceStatus {
                name: s.name,
                running: s.running,
                suggestion: s.suggestion,
            }).collect(),
            overall_status: result.overall_status,
        }
    }
}

#[tauri::command]
pub async fn diagnose_system(
    state: State<'_, AppState>,
) -> Result<SerializableDiagResult, String> {
    let manager = state.auth_manager.lock().map_err(|e| e.to_string())?;
    let result = manager.diagnose().map_err(|e| e.to_string())?;
    Ok(result.into())
}

/// 列出网络接口（占位实现）
#[tauri::command]
pub async fn list_network_interfaces() -> Result<Vec<String>, String> {
    // TODO: 实现跨平台网络接口列举
    Ok(vec![
        "eth0".to_string(),
        "wlan0".to_string(),
    ])
}
