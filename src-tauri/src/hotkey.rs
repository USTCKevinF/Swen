use crate::config::{get_config_value, set_config_value};
use crate::windows::{handle_text_selection_shortcut, handle_screenshot_ocr_shortcut, handle_app_activation_shortcut};
use crate::APP;
use log::{info, warn};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};

fn register_global_shortcut<F>(
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
            match get_config_value(name) {
                Some(v) => v.as_str().unwrap().to_string(),
                None => {
                    set_config_value(name, "");
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
pub fn initialize_shortcut_handler(shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match shortcut {
        "shortcut_text_selection" => {
            register_global_shortcut(app_handle, "shortcut_text_selection", handle_text_selection_shortcut, "")?
        }
        "shortcut_screenshot_ocr" => {
            register_global_shortcut(app_handle, "shortcut_screenshot_ocr", handle_screenshot_ocr_shortcut, "")?
        }
        "all" => {
            register_global_shortcut(app_handle, "shortcut_text_selection", handle_text_selection_shortcut, "")?;
            register_global_shortcut(app_handle, "shortcut_screenshot_ocr", handle_screenshot_ocr_shortcut, "")?;
            register_global_shortcut(app_handle, "shortcut_app_activation", handle_app_activation_shortcut, "")?;
        }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
pub fn update_shortcut_from_frontend(name: &str, shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match name {
        "shortcut_text_selection" => {
            register_global_shortcut(app_handle, "shortcut_text_selection", handle_text_selection_shortcut, shortcut)?
        }
        "shortcut_screenshot_ocr" => {
            register_global_shortcut(app_handle, "shortcut_screenshot_ocr", handle_screenshot_ocr_shortcut, shortcut)?
        }
        "shortcut_app_activation" => {
            register_global_shortcut(app_handle, "shortcut_app_activation", handle_app_activation_shortcut, shortcut)?
        }
        _ => {}
    }
    Ok(())
}

// 初始化快捷键设置，如果为空则设置默认值
pub fn initialize_default_shortcuts() {
    // 初始化选择文本快捷键
    match get_config_value("shortcut_text_selection") {
        Some(v) => {
            if v.as_str().map_or(true, |s: &str| s.is_empty()) {
                info!("初始化选择文本快捷键为 Option+E");
                set_config_value("shortcut_text_selection", "Option+E");
            }
        },
        None => {
            info!("设置选择文本快捷键默认值为 Option+E");
            set_config_value("shortcut_text_selection", "Option+E");
        }
    }
    
    // 初始化截屏快捷键
    match get_config_value("shortcut_screenshot_ocr") {
        Some(v) => {
            if v.as_str().map_or(true, |s| s.is_empty()) {
                info!("初始化截屏快捷键为 Option+W");
                set_config_value("shortcut_screenshot_ocr", "Option+W");
            }
        },
        None => {
            info!("设置截屏快捷键默认值为 Option+W");
            set_config_value("shortcut_screenshot_ocr", "Option+W");
        }
    }

    match get_config_value("shortcut_app_activation") {
        Some(v) => {
            if v.as_str().map_or(true, |s| s.is_empty()) {
                info!("初始化调用随问快捷键为 Option+Q");
                set_config_value("shortcut_app_activation", "Option+Q");
            }
        },
        None => {
            info!("设置调用随问快捷键默认值为 Option+Q");
            set_config_value("shortcut_app_activation", "Option+Q");
        }
    }
    
}