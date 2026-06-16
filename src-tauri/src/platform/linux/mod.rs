mod networkmanager;
mod cert_store;
pub use cert_store::*;

use crate::platform::{Network802_1XManager, Credentials, AuthStatus, DiagResult, ServiceStatus};
use anyhow::Result;

pub struct LinuxNetworkManager;

impl Network802_1XManager for LinuxNetworkManager {
    fn enable_auth(&self, interface: &str, creds: &Credentials) -> Result<()> {
        tracing::info!("Linux: 配置 802.1X PEAP 认证 - 接口: {}, 用户: {}", interface, creds.username);

        match networkmanager::configure_via_nm(interface, &creds.username, &creds.password) {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::warn!("NetworkManager 配置失败: {}, 降级到 wpa_supplicant", e);
                networkmanager::configure_via_wpa_supplicant(interface, &creds.username, &creds.password)
            }
        }
    }

    fn disable_auth(&self, interface: &str) -> Result<()> {
        tracing::info!("Linux: 禁用 802.1X 认证 - 接口: {}", interface);

        match networkmanager::disable_via_nm(interface) {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::warn!("NetworkManager 禁用失败: {}, 降级到 wpa_supplicant", e);
                networkmanager::disable_via_wpa_supplicant(interface)
            }
        }
    }

    fn get_status(&self, interface: &str) -> Result<AuthStatus> {
        let status = networkmanager::get_connection_status(interface)?;

        if status.contains("activated") || status.contains("connected") {
            let ip = networkmanager::get_ip_address(interface).unwrap_or_else(|_| "N/A".to_string());
            Ok(AuthStatus::Connected {
                interface: interface.to_string(),
                ip,
            })
        } else if status.contains("activating") || status.contains("connecting") {
            Ok(AuthStatus::Connecting)
        } else {
            Ok(AuthStatus::Disconnected)
        }
    }

    fn diagnose(&self) -> Result<DiagResult> {
        let mut services = vec![];

        let nm_running = networkmanager::check_service_running("NetworkManager").unwrap_or(false);
        services.push(ServiceStatus {
            name: "NetworkManager".to_string(),
            running: nm_running,
            suggestion: if !nm_running {
                Some("运行: sudo systemctl start NetworkManager".to_string())
            } else {
                None
            },
        });

        let wpa_installed = networkmanager::check_package_installed("wpa_supplicant").unwrap_or(false);
        services.push(ServiceStatus {
            name: "wpa_supplicant".to_string(),
            running: wpa_installed,
            suggestion: if !wpa_installed {
                Some("Debian/Ubuntu: sudo apt install wpasupplicant\nCentOS: sudo yum install wpa_supplicant".to_string())
            } else {
                None
            },
        });

        let interfaces = networkmanager::list_interfaces().unwrap_or_default();
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
