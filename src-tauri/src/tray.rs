use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

use crate::windows::config_window;
use crate::windows::home_window;

// 初始化托盘
pub fn init_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let home_i = MenuItem::with_id(app, "home", "主页", true, None::<&str>)?;
    let config_i = MenuItem::with_id(app, "config", "配置", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i, &home_i, &config_i])?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "home" => {
                println!("Home menu item clicked");
                home_window();
            }
            "config" => {
                println!("Config menu item clicked");
                config_window();
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;
    Ok(())
}
