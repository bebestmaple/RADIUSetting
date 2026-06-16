#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod platform;
mod commands;

use tauri::Manager;

fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config = core::config_loader::load_config()
        .unwrap_or_else(|e| {
            tracing::warn!("Failed to load config: {}, using defaults", e);
            core::config_loader::default_config()
        });

    tracing::info!("Starting RADIUS Client v{}", config.version);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化应用状态
            app.manage(commands::AppState {
                auth_manager: std::sync::Mutex::new(core::auth_manager::AuthManager::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connect_auth,
            commands::disconnect_auth,
            commands::get_auth_status,
            commands::refresh_status,
            commands::trust_certificate,
            commands::list_trusted_certs,
            commands::revoke_certificate_trust,
            commands::diagnose_system,
            commands::list_network_interfaces,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
