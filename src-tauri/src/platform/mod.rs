use anyhow::Result;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "linux")]
pub mod linux;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum AuthStatus {
    Disconnected,
    Connecting,
    Connected { interface: String, ip: String },
    Failed(String),
}

#[derive(Debug)]
pub struct DiagResult {
    pub services: Vec<ServiceStatus>,
    pub overall_status: bool,
}

#[derive(Debug)]
pub struct ServiceStatus {
    pub name: String,
    pub running: bool,
    pub suggestion: Option<String>,
}

pub trait Network802_1XManager {
    fn enable_auth(&self, interface: &str, creds: &Credentials) -> Result<()>;
    fn disable_auth(&self, interface: &str) -> Result<()>;
    fn get_status(&self, interface: &str) -> Result<AuthStatus>;
    fn diagnose(&self) -> Result<DiagResult>;
}

#[cfg(target_os = "windows")]
pub type PlatformManager = windows::WindowsNetworkManager;

#[cfg(target_os = "macos")]
pub type PlatformManager = macos::MacOSNetworkManager;

#[cfg(target_os = "linux")]
pub type PlatformManager = linux::LinuxNetworkManager;
