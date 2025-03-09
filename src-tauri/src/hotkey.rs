use crate::config::{get, set};
use crate::windows::{selection_get, system_screenshot_hotkey, call_swen};
use crate::APP;
use log::{info, warn};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};

fn register_hotkey<F>(
    app_handle: &AppHandle,
    name: &str,
    handler: F,
    key: &str,
) -> Result<(), String>
where
    F: Fn(&AppHandle, &Shortcut, ShortcutEvent) + Send + Sync + 'static,
{
    let hotkey = {
        if key.is_empty() {
            match get(name) {
                Some(v) => v.as_str().unwrap().to_string(),
                None => {
                    set(name, "");
                    String::new()
                }
            }
        } else {
            key.to_string()
        }
    };

    if !hotkey.is_empty() {
        match app_handle.global_shortcut().on_shortcut(
            hotkey.as_str(),
            move |app_handle, shortcut, event| {
                handler(app_handle, shortcut, event);
            },
        ) {
            Ok(()) => {
                info!("Registered global shortcut: {} for {}", hotkey, name);
            }
            Err(e) => {
                warn!("Failed to register global shortcut: {} {:?}", hotkey, e);
                return Err(e.to_string());
            }
        };
    }
    Ok(())
}

// Register global shortcuts
pub fn init_register_shortcut(shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match shortcut {
        "hotkey_selection_get" => {
            register_hotkey(app_handle, "hotkey_selection_get", selection_get, "")?
        }
        "hotkey_ocr" => {
            register_hotkey(app_handle, "hotkey_ocr", system_screenshot_hotkey, "")?
        }
        "all" => {
            register_hotkey(app_handle, "hotkey_selection_get", selection_get, "")?;
            register_hotkey(app_handle, "hotkey_ocr", system_screenshot_hotkey, "")?;
            register_hotkey(app_handle, "hotkey_call_swen", call_swen, "")?;
        }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
pub fn register_shortcut_by_frontend(name: &str, shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match name {
        "hotkey_selection_get" => {
            register_hotkey(app_handle, "hotkey_selection_get", selection_get, shortcut)?
        }
        "hotkey_ocr" => {
            register_hotkey(app_handle, "hotkey_ocr", system_screenshot_hotkey, shortcut)?
        }
        "hotkey_call_swen" => {
            register_hotkey(app_handle, "hotkey_call_swen", call_swen, shortcut)?
        }
        _ => {}
    }
    Ok(())
}

// 初始化快捷键设置，如果为空则设置默认值
pub fn init_hotkey_config() {
    // 初始化选择文本快捷键
    match get("hotkey_selection_get") {
        Some(v) => {
            if v.as_str().map_or(true, |s: &str| s.is_empty()) {
                info!("初始化选择文本快捷键为 Option+E");
                set("hotkey_selection_get", "Option+E");
            }
        },
        None => {
            info!("设置选择文本快捷键默认值为 Option+E");
            set("hotkey_selection_get", "Option+E");
        }
    }
    
    // 初始化截屏快捷键
    match get("hotkey_ocr") {
        Some(v) => {
            if v.as_str().map_or(true, |s| s.is_empty()) {
                info!("初始化截屏快捷键为 Option+W");
                set("hotkey_ocr", "Option+W");
            }
        },
        None => {
            info!("设置截屏快捷键默认值为 Option+W");
            set("hotkey_ocr", "Option+W");
        }
    }

    match get("hotkey_call_swen") {
        Some(v) => {
            if v.as_str().map_or(true, |s| s.is_empty()) {
                info!("初始化调用随问快捷键为 Option+Q");
                set("hotkey_call_swen", "Option+Q");
            }
        },
        None => {
            info!("设置调用随问快捷键默认值为 Option+Q");
            set("hotkey_call_swen", "Option+Q");
        }
    }
    
}
