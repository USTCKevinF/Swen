use crate::APP;
use dirs::config_dir;
use log::info;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{Manager, Wry};
use tauri_plugin_store::Store;
use tauri_plugin_store::StoreExt;

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
    println!("store: {:?}", store.is_empty());
    if store.is_empty() {
        return true;
    }
    
    let api_key = store.get("llm.apiKey");
    let base_url = store.get("llm.baseURL");
    let model = store.get("llm.model");
    
    println!("api_key: {:?}", api_key);
    println!("base_url: {:?}", base_url);
    println!("model: {:?}", model);
    
    if api_key.is_none() || base_url.is_none() || model.is_none() {
        return true;
    }
    
    let api_key = api_key.unwrap();
    let base_url = base_url.unwrap();
    let model = model.unwrap();
    
    if api_key.is_null() || base_url.is_null() || model.is_null() || 
       api_key.as_str().map_or(true, |s| s.is_empty()) ||
       base_url.as_str().map_or(true, |s| s.is_empty()) ||
       model.as_str().map_or(true, |s| s.is_empty())
    {
        true
    } else {
        false
    }
}

