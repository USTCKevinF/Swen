use dirs::config_dir;
use log::info;
use std::path::PathBuf;
use tauri_plugin_store::StoreExt;
use tauri::{Manager, Wry};

use tauri_plugin_store::Store;
use std::sync::Arc;
use std::sync::Mutex;
use crate::APP;

pub struct StoreWrapper(pub Mutex<Arc<Store<Wry>>>);

#[tauri::command]
pub fn reload_store() {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    store.reload().unwrap();
}

pub fn init_config(app: &mut tauri::App) {
    let config_path = get_config_path(app);
    info!("Load config from: {:?}", config_path);
    let store = app.handle().store(config_path).unwrap();
    app.manage(StoreWrapper(Mutex::new(store)));
}

pub fn get_config_path(app: &mut tauri::App) -> PathBuf {
    let config_path = config_dir().unwrap();
    let config_path = config_path.join(app.config().identifier.clone());
    let config_path = config_path.join("config.json");
    info!("Load config from: {:?}", config_path);
    return config_path;
}
