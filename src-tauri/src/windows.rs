// use crate::config::get;
// use crate::config::set;
use crate::config::get;
use crate::ocr::system_ocr;
use crate::screenshot::system_screenshot;
use crate::StringWrapper;
use crate::APP;
use log::{info, warn};
use mouse_position::mouse_position::{Mouse, Position};
use serde_json::json;
use tauri::Listener;
use tauri::Manager;
use tauri::Monitor;
// use tauri::Runtime;
use tauri::WebviewWindow;
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutEvent, ShortcutState};

// Get monitor where the mouse is currently located
// Get daemon window instance
fn get_daemon_window() -> WebviewWindow {
    let app_handle = APP.get().unwrap();
    match app_handle.get_webview_window("daemon") {
        Some(v) => v,
        None => {
            warn!("Daemon window not found, create new daemon window!");
            tauri::WebviewWindowBuilder::new(
                app_handle,
                "daemon",
                tauri::WebviewUrl::App("daemon.html".into()),
            )
            .title("Daemon")
            .additional_browser_args("--disable-web-security")
            .visible(false)
            .build()
            .unwrap()
        }
    }
}

fn get_current_monitor(x: i32, y: i32) -> Monitor {
    info!("Mouse position: {}, {}", x, y);
    let daemon_window = get_daemon_window();
    let monitors = daemon_window.available_monitors().unwrap();

    for m in monitors {
        let size = m.size();
        let position = m.position();

        if x >= position.x
            && x <= (position.x + size.width as i32)
            && y >= position.y
            && y <= (position.y + size.height as i32)
        {
            info!("Current Monitor: {:?}", m);
            return m;
        }
    }
    warn!("Current Monitor not found, using primary monitor");
    daemon_window.primary_monitor().unwrap().unwrap()
}

// Creating a window on the mouse monitor
pub fn build_window(label: &str, title: &str) -> (WebviewWindow, bool) {
    let mouse_position = match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Position { x, y },
        Mouse::Error => {
            warn!("Mouse position not found, using (0, 0) as default");
            Position { x: 0, y: 0 }
        }
    };
    let current_monitor = get_current_monitor(mouse_position.x, mouse_position.y);
    let position = current_monitor.position();

    let app_handle = APP.get().unwrap();
    match app_handle.get_webview_window(label) {
        Some(v) => {
            info!("Window existence: {}", label);
            
            // 确保窗口在所有工作区可见
            v.set_visible_on_all_workspaces(true).unwrap();
            
            // 将窗口移动到当前鼠标所在的显示器位置
            // v.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
            //     position.x,
            //     position.y,
            // ))).unwrap();
            
            // 确保窗口在当前桌面上显示
            #[cfg(target_os = "macos")]
            {
                // macOS 特定处理：先隐藏再显示可以强制窗口在当前桌面显示
                v.hide().unwrap();
                v.show().unwrap();
            }
            
            // 设置窗口焦点
            v.set_focus().unwrap();
            
            (v, true)
        }
        None => {
            info!("Window not existence, Creating new window: {}", label);
            let mut builder = tauri::WebviewWindowBuilder::new(
                app_handle,
                label,
                tauri::WebviewUrl::App("index.html".into()),
            )
            .position(position.x.into(), position.y.into())
            .additional_browser_args("--disable-web-security")
            .focused(true)
            .title(title)
            .visible(false);

            #[cfg(target_os = "macos")]
            {
                builder = builder
                    .title_bar_style(tauri::TitleBarStyle::Overlay)
                    .hidden_title(true);
            }
            #[cfg(not(target_os = "macos"))]
            {
                builder = builder.transparent(true).decorations(false);
            }
            let window = builder.build().unwrap();
            let _ = window.current_monitor();
            (window, false)
        }
    }
}

pub fn config_window() {
    let (window, _exists) = build_window("settings", "Settings");
    window
        .set_min_size(Some(tauri::LogicalSize::new(800, 600)))
        .unwrap();
    window.set_size(tauri::LogicalSize::new(800, 600)).unwrap();
    window.center().unwrap();
}

pub fn home_window() -> (WebviewWindow, bool) {
    let (window, exists) = build_window("home", "Home");
    window
        .set_min_size(Some(tauri::LogicalSize::new(400, 300)))
        .unwrap();
    window.set_focus().unwrap();

    let remember_window_size = get("rememberSize").unwrap();
    if remember_window_size == false {
        window.set_size(tauri::LogicalSize::new(500, 400)).unwrap();
    }
    let binding = get("windowPosition").unwrap();
    let window_position = binding.as_str().unwrap();
    if window_position == "center" {
        window.center().unwrap();
    } else if window_position == "followMouse" {
        let mouse_position = match Mouse::get_mouse_position() {
            Mouse::Position { x, y } => Position { x, y },
            Mouse::Error => {
                warn!("Mouse position not found, using (0, 0) as default");
                Position { x: 0, y: 0 }
            }
        };
        window
            .set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
                mouse_position.x,
                mouse_position.y,
            )))
            .unwrap();
    } else if window_position == "remember" {
        if !exists {
            window.center().unwrap();
        }
    }
    window.show().unwrap();
    // window.set_closable(false).unwrap();
    // window.set_transparent_titlebar(true, true);
    (window, exists)
}

// pub fn screenshot_window() -> WebviewWindow {
//     let (window, _exists) = build_window("screenshot", "Screenshot");

//     window.set_skip_taskbar(true).unwrap();
//     #[cfg(target_os = "macos")]
//     {
//         let monitor = window.current_monitor().unwrap().unwrap();
//         let size = monitor.size();
//         window.set_decorations(false).unwrap();
//         window.set_size(*size).unwrap();
//     }

//     #[cfg(not(target_os = "macos"))]
//     window.set_fullscreen(true).unwrap();

//     window.set_focus().unwrap();
//     window.set_always_on_top(true).unwrap();
//     window
// }

pub fn selection_get(app_handle: &AppHandle, _shortcut: &Shortcut, event: ShortcutEvent) {
    match event.state() {
        ShortcutState::Pressed => {
            use get_selected_text::get_selected_text;
            match get_selected_text() {
                Ok(text) => {
                    if !text.trim().is_empty() {
                        let state: tauri::State<StringWrapper> = app_handle.state();
                        state.0.lock().unwrap().replace_range(.., &text);
                        info!("Selected text: {}", text);

                        let (window, exists) = home_window();
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();

                        let app_handle_clone = app_handle.clone();
                        let text_clone = text.clone();

                        if exists {
                            app_handle
                                .emit_to(
                                    "home",
                                    "update-input",
                                    json!({
                                        "payload": text,
                                        "requestId": timestamp
                                    }),
                                )
                                .unwrap();
                        } else {
                            window.once("home-ready", move |_| {
                                app_handle_clone
                                    .emit_to(
                                        "home",
                                        "update-input",
                                        json!({
                                            "payload": text_clone,
                                            "requestId": timestamp
                                        }),
                                    )
                                    .unwrap();
                            });
                            window.show().unwrap();
                        }
                    }
                }
                Err(e) => {
                    warn!("获取选中文本失败: {}", e);
                }
            }
        }
        ShortcutState::Released => {
            info!("Shortcut released");
        }
    }
}

// // use system screenshot
// pub fn system_screenshot_window() {
//     let app_handle = APP.get().unwrap();
//     if let Err(e) = system_screenshot() {
//         warn!("截图失败: {}", e);
//         return;
//     }
//     // 先创建窗口
//     let (window, exists) = home_window();

//     let app_handle_clone = app_handle.clone();
//     let state: tauri::State<StringWrapper> = app_handle_clone.state();
//     let ocr_result = match system_ocr(app_handle_clone.clone(), "auto") {
//         Ok(result) => {
//             info!("OCR Result: {}", result);
//             result
//         }
//         Err(e) => {
//             warn!("OCR失败: {}", e);
//             return;
//         }
//     };

//     state.0.lock().unwrap().replace_range(.., &ocr_result);

//     // 添加时间戳
//     let timestamp = std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
//         .as_millis();

//     let ocr_result_clone = ocr_result.clone();

//     if exists {
//         app_handle
//             .emit_to(
//                 "home",
//                 "update-input",
//                 json!({
//                     "payload": ocr_result,
//                     "requestId": timestamp
//                 }),
//             )
//             .unwrap();
//     } else {
//         window.once("home-ready", move |_| {
//             app_handle_clone
//                 .emit_to(
//                     "home",
//                     "update-input",
//                     json!({
//                         "payload": ocr_result_clone,
//                         "requestId": timestamp
//                     }),
//                 )
//                 .unwrap();
//         });
//         window.show().unwrap();
//     }
// }

// use system screenshot
pub fn system_screenshot_hotkey(
    app_handle: &AppHandle,
    _shortcut: &Shortcut,
    event: ShortcutEvent,
) {
    match event.state() {
        ShortcutState::Pressed => {
            if let Err(e) = system_screenshot() {
                warn!("截图失败: {}", e);
                return;
            }
            // 先创建窗口
            
            let app_handle_clone = app_handle.clone();
            let state: tauri::State<StringWrapper> = app_handle_clone.state();
            let ocr_result = match system_ocr(app_handle_clone.clone(), "auto") {
                Ok(result) => {
                    info!("OCR Result: {}", result);
                    result
                }
                Err(e) => {
                    warn!("OCR失败: {}", e);
                    return;
                }
            };
            
            state.0.lock().unwrap().replace_range(.., &ocr_result);
            
            // 添加时间戳
            let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
            let ocr_result_clone = ocr_result.clone();
            if ocr_result.is_empty() {
                return;
            }
        
            let (window, exists) = home_window();
            if exists {
                app_handle
                    .emit_to(
                        "home",
                        "update-input",
                        json!({
                            "payload": ocr_result,
                            "requestId": timestamp
                        }),
                    )
                    .unwrap();
            } else {
                window.once("home-ready", move |_| {
                    app_handle_clone
                        .emit_to(
                            "home",
                            "update-input",
                            json!({
                                "payload": ocr_result_clone,
                                "requestId": timestamp
                            }),
                        )
                        .unwrap();
                });
                window.show().unwrap();
            }
        }
        ShortcutState::Released => {
            info!("Shortcut released");
        }
    }
}

// use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSWindowTitleVisibility};

// pub trait WindowExt {
//     #[cfg(target_os = "macos")]
//     fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
// }

// impl<R: Runtime> WindowExt for WebviewWindow<R> {
//     #[cfg(target_os = "macos")]
//     fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
//         unsafe {
//             let id = self.ns_window().unwrap() as cocoa::base::id;
//             NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
//             let mut style_mask = id.styleMask();
//             style_mask.set(
//                 NSWindowStyleMask::NSFullSizeContentViewWindowMask,
//                 title_transparent,
//             );

//             if remove_tool_bar {
//                 style_mask.remove(
//                     NSWindowStyleMask::NSClosableWindowMask
//                         | NSWindowStyleMask::NSMiniaturizableWindowMask
//                         | NSWindowStyleMask::NSResizableWindowMask,
//                 );
//             }

//             id.setStyleMask_(style_mask);

//             id.setTitleVisibility_(if title_transparent {
//                 NSWindowTitleVisibility::NSWindowTitleHidden
//             } else {
//                 NSWindowTitleVisibility::NSWindowTitleVisible
//             });

//             id.setTitlebarAppearsTransparent_(if title_transparent {
//                 cocoa::base::YES
//             } else {
//                 cocoa::base::NO
//             });
//         }
//     }
// }

// use manual screenshot display page
// pub fn ocr_get() {
//     let window = screenshot_window();
//     let window_ = window.clone();
//     window.listen("success", move |event| {
//         let app_handle = APP.get().unwrap();
//         let cache_dir = cache_dir().expect("无法获取缓存目录");
//         let app_cache_dir = cache_dir.join(&app_handle.config().identifier);

//         // 确保目录存在
//         if !app_cache_dir.exists() {
//             std::fs::create_dir_all(&app_cache_dir).expect("无法创建缓存目录");
//         }

//         // 检查裁剪后的图片是否存在
//         let cut_image_path = app_cache_dir.join("YYSM_Tool_screenshot_cut.png");
//         if !cut_image_path.exists() {
//             warn!("裁剪后的图片不存在: {:?}", cut_image_path);
//             return;
//         }

//         let state: tauri::State<StringWrapper> = app_handle.state();

//         // 添加日志以便调试
//         info!("开始进行OCR识别，图片路径: {:?}", cut_image_path);

//         let ocr_result = match system_ocr(app_handle.clone(), "auto") {
//             Ok(result) => result,
//             Err(e) => {
//                 warn!("OCR 失败: {}", e);
//                 return;
//             }
//         };

//         info!("OCR Result: {}", ocr_result);
//         state.0.lock().unwrap().replace_range(.., &ocr_result);

//         app_handle.emit_to("home", "update-input", ocr_result).unwrap();
//         home_window();
//         window_.unlisten(event.id())
//     });

// }

// pub fn ocr_get_hotkey(_app_handle: &AppHandle, _shortcut: &Shortcut, _event: ShortcutEvent) {
//     let window = screenshot_window();
//     let window_ = window.clone();
//     // 克隆 app_handle 以便在闭包中使用
//     window.listen("success", move |event| {
//         let app_handle = APP.get().unwrap();
//         let cache_dir = cache_dir().expect("无法获取缓存目录");
//         let app_cache_dir = cache_dir.join(&app_handle.config().identifier);

//         // 确保目录存在
//         if !app_cache_dir.exists() {
//             std::fs::create_dir_all(&app_cache_dir).expect("无法创建缓存目录");
//         }

//         // 检查裁剪后的图片是否存在
//         let cut_image_path = app_cache_dir.join("YYSM_Tool_screenshot_cut.png");
//         if !cut_image_path.exists() {
//             warn!("裁剪后的图片不存在: {:?}", cut_image_path);
//             return;
//         }

//         let state: tauri::State<StringWrapper> = app_handle.state();

//         // 添加日志以便调试
//         info!("开始进行OCR识别，图片路径: {:?}", cut_image_path);

//         let ocr_result = match system_ocr(app_handle.clone(), "auto") {
//             Ok(result) => result,
//             Err(e) => {
//                 warn!("OCR 失败: {}", e);
//                 return;
//             }
//         };

//         info!("OCR Result: {}", ocr_result);
//         state.0.lock().unwrap().replace_range(.., &ocr_result);

//         app_handle.emit_to("home", "update-input", ocr_result).unwrap();
//         home_window();
//         window_.unlisten(event.id())
//     });

// }
