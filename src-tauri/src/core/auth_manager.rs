use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::platform::{AuthStatus as PlatformAuthStatus, Credentials, Network802_1XManager, PlatformManager};
use crate::core::cert_validator::{CertificateInfo, is_certificate_trusted};

/// 认证事件
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthEvent {
    StateChanged { state: AuthState },
    CertificatePrompt { cert_info: CertificateInfo },
    Error { message: String },
    Progress { message: String },
}

/// 认证状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status")]
pub enum AuthState {
    Disconnected,
    Connecting { interface: String },
    CertificatePrompt { cert_fingerprint: String },
    Connected { interface: String, ip: String, connected_at: String },
    Failed { error: String },
}

/// 认证管理器
pub struct AuthManager {
    state: Arc<Mutex<AuthState>>,
    platform_manager: PlatformManager,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AuthState::Disconnected)),
            platform_manager: PlatformManager::default(),
        }
    }

    /// 获取当前状态
    pub fn get_state(&self) -> AuthState {
        self.state.lock().unwrap().clone()
    }

    /// 设置状态
    fn set_state(&self, new_state: AuthState) {
        let mut state = self.state.lock().unwrap();
        *state = new_state;
    }

    /// 启用认证
    pub fn enable_auth(&self, interface: &str, username: &str, password: &str) -> Result<()> {
        self.set_state(AuthState::Connecting {
            interface: interface.to_string(),
        });

        let creds = Credentials {
            username: username.to_string(),
            password: password.to_string(),
        };

        self.platform_manager.enable_auth(interface, &creds)?;

        let status = self.platform_manager.get_status(interface)?;

        let new_state = match status {
            PlatformAuthStatus::Connected { interface, ip } => {
                AuthState::Connected {
                    interface,
                    ip,
                    connected_at: chrono::Utc::now().to_rfc3339(),
                }
            }
            PlatformAuthStatus::Connecting => {
                AuthState::Connecting {
                    interface: interface.to_string(),
                }
            }
            PlatformAuthStatus::Failed(err) => {
                AuthState::Failed { error: err }
            }
            PlatformAuthStatus::Disconnected => {
                AuthState::Disconnected
            }
        };

        self.set_state(new_state);
        Ok(())
    }

    /// 禁用认证
    pub fn disable_auth(&self, interface: &str) -> Result<()> {
        self.platform_manager.disable_auth(interface)?;
        self.set_state(AuthState::Disconnected);
        Ok(())
    }

    /// 刷新状态
    pub fn refresh_status(&self, interface: &str) -> Result<AuthState> {
        let status = self.platform_manager.get_status(interface)?;

        let new_state = match status {
            PlatformAuthStatus::Connected { interface, ip } => {
                AuthState::Connected {
                    interface,
                    ip,
                    connected_at: chrono::Utc::now().to_rfc3339(),
                }
            }
            PlatformAuthStatus::Connecting => {
                AuthState::Connecting {
                    interface: interface.to_string(),
                }
            }
            PlatformAuthStatus::Failed(err) => {
                AuthState::Failed { error: err }
            }
            PlatformAuthStatus::Disconnected => {
                AuthState::Disconnected
            }
        };

        self.set_state(new_state.clone());
        Ok(new_state)
    }

    /// 处理证书信任
    pub fn handle_certificate_trust(&self, fingerprint: &str) -> Result<()> {
        if is_certificate_trusted(fingerprint)? {
            tracing::info!("证书已在信任列表中: {}", fingerprint);
            return Ok(());
        }

        self.set_state(AuthState::CertificatePrompt {
            cert_fingerprint: fingerprint.to_string(),
        });

        Ok(())
    }

    /// 确认信任证书后继续连接
    pub fn trust_and_continue(&self, cert_info: &CertificateInfo, interface: &str, username: &str, password: &str) -> Result<()> {
        crate::core::cert_validator::trust_certificate(cert_info)?;
        self.enable_auth(interface, username, password)?;
        Ok(())
    }

    /// 诊断
    pub fn diagnose(&self) -> Result<crate::platform::DiagResult> {
        self.platform_manager.diagnose()
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        let manager = AuthManager::new();
        assert_eq!(manager.get_state(), AuthState::Disconnected);

        manager.set_state(AuthState::Connecting {
            interface: "eth0".to_string(),
        });
        assert!(matches!(manager.get_state(), AuthState::Connecting { .. }));

        manager.set_state(AuthState::Connected {
            interface: "eth0".to_string(),
            ip: "192.168.1.100".to_string(),
            connected_at: chrono::Utc::now().to_rfc3339(),
        });
        assert!(matches!(manager.get_state(), AuthState::Connected { .. }));

        manager.set_state(AuthState::Failed {
            error: "认证失败".to_string(),
        });
        assert!(matches!(manager.get_state(), AuthState::Failed { .. }));

        manager.set_state(AuthState::CertificatePrompt {
            cert_fingerprint: "AA:BB:CC:DD".to_string(),
        });
        assert!(matches!(manager.get_state(), AuthState::CertificatePrompt { .. }));
    }

    #[test]
    fn test_state_clone() {
        let state = AuthState::Connected {
            interface: "eth0".to_string(),
            ip: "192.168.1.100".to_string(),
            connected_at: "2026-06-15T10:00:00Z".to_string(),
        };
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let manager = Arc::new(AuthManager::new());
        let manager_clone = Arc::clone(&manager);

        let handle = thread::spawn(move || {
            manager_clone.set_state(AuthState::Connecting {
                interface: "wlan0".to_string(),
            });
        });

        handle.join().unwrap();

        assert!(matches!(manager.get_state(), AuthState::Connecting { .. }));
    }
}
