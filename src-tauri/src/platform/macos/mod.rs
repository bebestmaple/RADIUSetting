mod eapolclient;
mod keychain;

pub use keychain::*;

use crate::platform::{Network802_1XManager, Credentials, AuthStatus, DiagResult, ServiceStatus};
use anyhow::Result;

pub struct MacOSNetworkManager;

impl Default for MacOSNetworkManager {
    fn default() -> Self {
        Self
    }
}

impl Network802_1XManager for MacOSNetworkManager {
    fn enable_auth(&self, interface: &str, creds: &Credentials) -> Result<()> {
        tracing::info!("macOS: 配置 802.1X PEAP 认证 - 接口: {}, 用户: {}", interface, creds.username);
        eapolclient::configure_peap(interface, &creds.username, &creds.password)
    }

    fn disable_auth(&self, interface: &str) -> Result<()> {
        tracing::info!("macOS: 禁用 802.1X 认证 - 接口: {}", interface);
        eapolclient::disable_802_1x(interface)
    }

    fn get_status(&self, interface: &str) -> Result<AuthStatus> {
        let output = eapolclient::get_interface_status(interface)?;

        if output.contains("authenticated") || output.contains("Authenticated") {
            Ok(AuthStatus::Connected {
                interface: interface.to_string(),
                ip: "查看 ifconfig 获取".to_string(),
            })
        } else if output.contains("authenticating") {
            Ok(AuthStatus::Connecting)
        } else {
            Ok(AuthStatus::Disconnected)
        }
    }

    fn diagnose(&self) -> Result<DiagResult> {
        let mut services = vec![];

        let eapol_running = eapolclient::check_eapolclient_running().unwrap_or(false);
        services.push(ServiceStatus {
            name: "eapolclient".to_string(),
            running: eapol_running,
            suggestion: if !eapol_running {
                Some("检查 EAPoL 客户端是否安装".to_string())
            } else {
                None
            },
        });

        let interfaces = eapolclient::list_interfaces().unwrap_or_default();
        services.push(ServiceStatus {
            name: "网络接口".to_string(),
            running: !interfaces.is_empty(),
            suggestion: if interfaces.is_empty() {
                Some("未检测到可用网络接口".to_string())
            } else {
                None
            },
        });

        let overall = services.iter().all(|s| s.running);
        Ok(DiagResult { services, overall_status: overall })
    }
}
