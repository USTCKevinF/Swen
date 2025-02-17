// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod config;
mod hotkey;
mod tray;
mod windows;
mod llm;
use hotkey::init_register_shortcut;
use log::info;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tauri::Manager;
pub struct StringWrapper(pub Mutex<String>);
// Global AppHandle
pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            info!("============== Start App ==============");
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                let trusted =
                    macos_accessibility_client::accessibility::application_is_trusted_with_prompt();
                info!("MacOS Accessibility Trusted: {}", trusted);
            }
            app.manage(StringWrapper(Mutex::new("".to_string())));

            tray::init_tray(&app.handle())?;
            // Global AppHandle
            APP.get_or_init(|| app.handle().clone());

            config::init_config(app);
            // Register Global Shortcut
            match init_register_shortcut("all") {
                Ok(()) => {}
                Err(e) => info!("Failed to register global shortcut: {}", e),
            }
            // let config_path = get_config_path(app);
            // Check First Run
            // if is_first_run() {
            //     // Open Config Window
            //     info!("First Run, opening config window");
            //     config_window();
            // }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            config::reload_store,
            hotkey::register_shortcut_by_frontend,
            llm::receive_stream,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
