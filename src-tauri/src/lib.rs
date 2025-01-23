// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod config;
mod tray;
mod windows;
use config::*;
use windows::*;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tauri::Manager;
use log::info;
use tauri_plugin_store::Store;
use tauri::Wry;
use tauri_plugin_store::StoreExt;
use serde_json::json;
pub struct StringWrapper(pub Mutex<String>);
// Global AppHandle
pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();
// 定义 StoreWrapper
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(StringWrapper(Mutex::new("".to_string())));

            tray::init_tray(&app.handle())?;
            // Global AppHandle
            APP.get_or_init(|| app.handle().clone());

            // let config_path = get_config_path(app);
            // Check First Run
            // if is_first_run() {
            //     // Open Config Window
            //     info!("First Run, opening config window");
            //     config_window();
            // }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}