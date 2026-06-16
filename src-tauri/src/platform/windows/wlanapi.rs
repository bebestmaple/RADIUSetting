use anyhow::{Result, bail};
use std::ptr::null_mut;

pub fn configure_peap_via_wlanapi(interface: &str, username: &str, password: &str) -> Result<()> {
    bail!("WLAN API 实现待完善，当前使用 netsh 作为备用方案");
}

pub fn remove_profile_via_wlanapi(interface: &str) -> Result<()> {
    bail!("WLAN API 实现待完善，当前使用 netsh 作为备用方案");
}

pub fn get_wlan_status(interface: &str) -> Result<String> {
    bail!("WLAN API 实现待完善，当前使用 netsh 作为备用方案");
}
