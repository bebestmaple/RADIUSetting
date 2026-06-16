use anyhow::{Result, bail};
use std::ptr::null_mut;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winsvc::*;
use winapi::shared::winerror::ERROR_SERVICE_ALREADY_RUNNING;

fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

pub fn ensure_service_running(service_name: &str) -> Result<()> {
    unsafe {
        let sc_manager = OpenSCManagerW(null_mut(), null_mut(), SC_MANAGER_CONNECT);
        if sc_manager.is_null() {
            bail!("无法打开服务控制管理器");
        }

        let service_name_wide = to_wide_string(service_name);
        let service = OpenServiceW(
            sc_manager,
            service_name_wide.as_ptr(),
            SERVICE_QUERY_STATUS | SERVICE_START,
        );

        if service.is_null() {
            CloseServiceHandle(sc_manager);
            bail!("无法打开服务: {}", service_name);
        }

        let mut status: SERVICE_STATUS = std::mem::zeroed();
        if QueryServiceStatus(service, &mut status) == 0 {
            CloseServiceHandle(service);
            CloseServiceHandle(sc_manager);
            bail!("无法查询服务状态: {}", service_name);
        }

        if status.dwCurrentState != SERVICE_RUNNING {
            if StartServiceW(service, 0, null_mut()) == 0 {
                let err = winapi::um::errhandlingapi::GetLastError();
                if err != ERROR_SERVICE_ALREADY_RUNNING {
                    CloseServiceHandle(service);
                    CloseServiceHandle(sc_manager);
                    bail!("启动服务失败: {} (错误代码: {})", service_name, err);
                }
            }

            for _ in 0..30 {
                std::thread::sleep(std::time::Duration::from_millis(100));
                if QueryServiceStatus(service, &mut status) != 0 {
                    if status.dwCurrentState == SERVICE_RUNNING {
                        break;
                    }
                }
            }

            if status.dwCurrentState != SERVICE_RUNNING {
                CloseServiceHandle(service);
                CloseServiceHandle(sc_manager);
                bail!("服务启动超时: {}", service_name);
            }
        }

        CloseServiceHandle(service);
        CloseServiceHandle(sc_manager);
        Ok(())
    }
}

pub fn check_service_status(service_name: &str) -> Result<bool> {
    unsafe {
        let sc_manager = OpenSCManagerW(null_mut(), null_mut(), SC_MANAGER_CONNECT);
        if sc_manager.is_null() {
            bail!("无法打开服务控制管理器");
        }

        let service_name_wide = to_wide_string(service_name);
        let service = OpenServiceW(sc_manager, service_name_wide.as_ptr(), SERVICE_QUERY_STATUS);

        if service.is_null() {
            CloseServiceHandle(sc_manager);
            return Ok(false);
        }

        let mut status: SERVICE_STATUS = std::mem::zeroed();
        let result = if QueryServiceStatus(service, &mut status) != 0 {
            status.dwCurrentState == SERVICE_RUNNING
        } else {
            false
        };

        CloseServiceHandle(service);
        CloseServiceHandle(sc_manager);
        Ok(result)
    }
}
