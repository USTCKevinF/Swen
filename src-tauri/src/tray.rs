use tauri::{
    AppHandle,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

// 初始化托盘
pub fn init_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let select_i = MenuItem::with_id(app, "select", "划文本", true, None::<&str>)?;
    let screenshot_i = MenuItem::with_id(app, "screenshot", "截图选文本", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i, &select_i, &screenshot_i])?;
    
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            "translate" => {
                // TODO: 实现翻译功能
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;
    Ok(())
}
