use crate::config::{get, set};
use crate::windows::selection_get;
use crate::APP;
use log::{info, warn};
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent};

fn register_hotkey<F>(app_handle: &AppHandle, name: &str, handler: F, key: &str) -> Result<(), String>
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
        match app_handle
            .global_shortcut()
            .on_shortcut(hotkey.as_str(), move |app_handle, shortcut, event| {
                handler(app_handle, shortcut, event);
            })
        {
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
        "hotkey_selection_get" => register_hotkey(
            app_handle,
            "hotkey_selection_get",
            selection_get,
            "",
        )?,
        "all" => {
            register_hotkey(
                app_handle,
                "hotkey_selection_get",
                selection_get,
                "",
            )?;
        }
        _ => {}
    }
    Ok(())
}

#[tauri::command]
pub fn register_shortcut_by_frontend(name: &str, shortcut: &str) -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    match name {
        "hotkey_selection_get" => register_hotkey(
            app_handle,
            "hotkey_selection_get",
            selection_get,
            shortcut,
        )?,
        _ => {}
    }
    Ok(())
}
