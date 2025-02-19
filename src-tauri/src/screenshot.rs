use log::info;
use log::error;

#[tauri::command]
pub fn screenshot(x: i32, y: i32) {
    use crate::APP;
    use dirs::cache_dir;
    use screenshots::{Screen, Compression};
    use std::fs;
    use std::io::Write;
    info!("开始截图，位置: x={}, y={}", x, y);
    let screens = Screen::all().unwrap();
    for screen in screens {
        let info = screen.display_info;
        info!("检测到屏幕: {:?}", info);
        if info.x == x && info.y == y {
            let handle = APP.get().unwrap();
            let mut app_cache_dir_path = cache_dir().expect("获取缓存目录失败");
            app_cache_dir_path.push(&handle.config().identifier.clone());
            info!("缓存目录路径: {:?}", app_cache_dir_path);
            
            if !app_cache_dir_path.exists() {
                fs::create_dir_all(&app_cache_dir_path).expect("创建缓存目录失败");
            }
            app_cache_dir_path.push("YYSM_Tool_screenshot.png");
            info!("截图保存路径: {:?}", app_cache_dir_path);

            let image = screen.capture().unwrap();
            info!("截图成功");
            
            // 将图片编码为 PNG 格式
            match image.to_png(Compression::Fast) {
                Ok(buffer) => {
                    match fs::write(&app_cache_dir_path, buffer) {
                        Ok(_) => info!("截图保存成功"),
                        Err(e) => error!("截图保存失败: {:?}", e),
                    }
                },
                Err(e) => error!("PNG 编码失败: {:?}", e),
            }
            break;
        }
    }
}

#[tauri::command]
pub fn cut_image(left: u32, top: u32, width: u32, height: u32, app_handle: tauri::AppHandle) {
    use dirs::cache_dir;
    use image::GenericImage;
    info!("Cut image: {}x{}+{}+{}", width, height, left, top);
    let mut app_cache_dir_path = cache_dir().expect("Get Cache Dir Failed");
    app_cache_dir_path.push(&app_handle.config().identifier.clone());
    app_cache_dir_path.push("YYSM_Tool_screenshot.png");
    if !app_cache_dir_path.exists() {
        return;
    }
    let mut img = match image::open(&app_cache_dir_path) {
        Ok(v) => v,
        Err(e) => {
            error!("error {:?}", e.to_string());
            return;
        }
    };
    let img2 = img.sub_image(left, top, width, height);
    app_cache_dir_path.pop();
    app_cache_dir_path.push("YYSM_Tool_screenshot_cut.png");
    match img2.to_image().save(&app_cache_dir_path) {
        Ok(_) => {}
        Err(e) => {
            error!("error cut image {:?}", e.to_string());
        }
    }
}