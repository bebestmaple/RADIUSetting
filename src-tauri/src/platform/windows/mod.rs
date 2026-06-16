mod service_manager;
mod netsh;
mod wlanapi;
mod cert_store;

pub use cert_store::*;

use crate::platform::{Network802_1XManager, Credentials, AuthStatus, DiagResult, ServiceStatus};
use anyhow::Result;

pub struct WindowsNetworkManager;

impl Default for WindowsNetworkManager {
    fn default() -> Self {
        Self
    }
}

impl Network802_1XManager for WindowsNetworkManager {
    fn enable_auth(&self, interface: &str, creds: &Credentials) -> Result<()> {
        service_manager::ensure_service_running("dot3svc")?;
        service_manager::ensure_service_running("EapHost")?;

        let os_version = get_os_version();

        if os_version >= 8 {
            match wlanapi::configure_peap_via_wlanapi(interface, &creds.username, &creds.password) {
                Ok(_) => Ok(()),
                Err(_) => {
                    netsh::configure_peap_via_netsh(interface, &creds.username)
                }
            }
        } else {
            netsh::configure_peap_via_netsh(interface, &creds.username)
        }
    }

    fn disable_auth(&self, interface: &str) -> Result<()> {
        let os_version = get_os_version();

        if os_version >= 8 {
            match wlanapi::remove_profile_via_wlanapi(interface) {
                Ok(_) => Ok(()),
                Err(_) => netsh::remove_profile_via_netsh(interface),
            }
        } else {
            netsh::remove_profile_via_netsh(interface)
        }
    }

    fn get_status(&self, interface: &str) -> Result<AuthStatus> {
        let status_output = netsh::get_lan_status(interface)?;

        if status_output.contains("已连接") || status_output.contains("Connected") {
            Ok(AuthStatus::Connected {
                interface: interface.to_string(),
                ip: "N/A".to_string(),
            })
        } else if status_output.contains("正在连接") || status_output.contains("Connecting") {
            Ok(AuthStatus::Connecting)
        } else {
            Ok(AuthStatus::Disconnected)
        }
    }

    fn diagnose(&self) -> Result<DiagResult> {
        let mut services = vec![];

        for svc in &["dot3svc", "WlanSvc", "EapHost"] {
            let running = service_manager::check_service_status(svc).unwrap_or(false);
            services.push(ServiceStatus {
                name: svc.to_string(),
                running,
                suggestion: if !running {
                    Some("点击'自动启动服务'按钮".to_string())
                } else {
                    None
                },
            });
        }

        let overall = services.iter().all(|s| s.running);
        Ok(DiagResult { services, overall_status: overall })
    }
}

fn get_os_version() -> u32 {
    use winapi::um::sysinfoapi::GetVersionExW;
    use winapi::um::winnt::OSVERSIONINFOEXW;
    use std::mem;

    unsafe {
        let mut version_info: OSVERSIONINFOEXW = mem::zeroed();
        version_info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEXW>() as u32;

        if GetVersionExW(&mut version_info as *mut _ as *mut _) != 0 {
            version_info.dwMajorVersion
        } else {
            10
        }
    }
}
