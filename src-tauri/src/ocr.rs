use dirs::cache_dir;
use tauri::Manager;
use std::path::PathBuf;
use log::info;

#[tauri::command(async)]
#[cfg(target_os = "macos")]
pub fn system_ocr(app_handle: tauri::AppHandle, lang: &str) -> Result<String, String> {
    let mut app_cache_dir_path = cache_dir().expect("Get Cache Dir Failed");
    app_cache_dir_path.push(&app_handle.config().identifier.clone());
    app_cache_dir_path.push("YYSM_Tool_screenshot_cut.png");
    info!("OCR 缓存目录: {:?}", app_cache_dir_path);
    let arch = std::env::consts::ARCH;
    let resource_dir: PathBuf = app_handle
        .path()
        .resource_dir()
        .map_err(|_| "Failed to get resource directory".to_string())?;
    let bin_path = resource_dir.join(format!("resources/ocr-{arch}-apple-darwin"));

    info!("OCR 二进制文件路径: {:?}", bin_path);
    match std::process::Command::new("chmod")
        .arg("+x")
        .arg(bin_path.to_str().unwrap())
        .output()
    {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }
    info!("OCR 命令: {:?}", bin_path);
    let output = match std::process::Command::new(bin_path)
        .arg(app_cache_dir_path.to_str().unwrap())
        .arg(lang)
        .output()
    {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    info!("OCR 输出: {:?}", output);
    if output.status.success() {
        let content = String::from_utf8(output.stdout).unwrap_or_default();
        Ok(content)
    } else {
        let content = String::from_utf8(output.stderr).unwrap_or_default();
        Err(content)
    }
}