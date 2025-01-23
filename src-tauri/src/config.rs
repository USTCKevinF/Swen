use crate::APP;
use dirs::config_dir;
use log::info;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use tauri::Wry;
use tauri_plugin_store::Store;

pub struct StoreWrapper(pub Mutex<Store<Wry>>);

pub fn get(key: &str) -> Option<Value> {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    match store.get(key) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

pub fn set<T: serde::ser::Serialize>(key: &str, value: T) {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    store.set(key.to_string(), json!(value));
    store.save().unwrap();
}

pub fn is_first_run() -> bool {
    let state = APP.get().unwrap().state::<StoreWrapper>();
    let store = state.0.lock().unwrap();
    store.is_empty()
}

pub fn get_config_path(app: &mut tauri::App) -> PathBuf {
    let config_path = config_dir().unwrap();
    let config_path = config_path.join(app.config().identifier.clone());
    let config_path = config_path.join("config.json");
    info!("Load config from: {:?}", config_path);
    return config_path;
}
