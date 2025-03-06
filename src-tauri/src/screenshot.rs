use crate::APP;
use dirs::cache_dir;
use log::error;
use log::info;
use std::{thread::sleep, time::Duration};

#[tauri::command]
pub fn screenshot() -> Result<(), String> {
    let handle = APP.get().unwrap();
    let mut app_cache_dir_path = cache_dir().expect("获取缓存目录失败");
    app_cache_dir_path.push(&handle.config().identifier.clone());
    let screenshot_path = app_cache_dir_path.join("YYSM_Tool_screenshot.png");

    // 确保目录存在
    if !app_cache_dir_path.exists() {
        std::fs::create_dir_all(&app_cache_dir_path).map_err(|e| e.to_string())?;
    }

    // 删除旧截图文件（如果存在）
    let _ = std::fs::remove_file(&screenshot_path);

    std::process::Command::new("screencapture")
        .arg("-x") // 添加静默模式参数
        .arg(screenshot_path.to_str().unwrap())
        .output()
        .map_err(|e| e.to_string())?;

    // 等待文件生成（最多等待2秒）
    let max_retries = 20;
    let mut retries = 0;
    while retries < max_retries {
        if screenshot_path.exists() {
            break;
        }
        sleep(Duration::from_millis(100));
        retries += 1;
    }

    if !screenshot_path.exists() {
        return Err("截图文件未生成".into());
    }

    Ok(())
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

pub fn system_screenshot() -> Result<(), String> {
    let app_handle = APP.get().unwrap();
    let cache_dir = cache_dir().expect("无法获取缓存目录");
    let app_cache_dir = cache_dir.join(&app_handle.config().identifier);
    let screenshot_path = app_cache_dir.join("YYSM_Tool_screenshot_cut.png");
    // 删除旧截图文件（如果存在）
    let _ = std::fs::remove_file(&screenshot_path);
    std::process::Command::new("screencapture")
        .arg("-i")
        .arg(screenshot_path.to_str().unwrap())
        .output()
        .map_err(|e| e.to_string())?;

    Ok(())
}
