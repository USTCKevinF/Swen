use futures_util::StreamExt;
use tauri_plugin_http::reqwest::{header, Client};
use std::str::from_utf8;
use tauri::{AppHandle, Runtime, Emitter};
use serde_json;

#[derive(Clone, serde::Serialize)]
pub struct StreamPayload {
    pub message: String,
    pub prompt: String,
}

#[tauri::command]
pub async fn receive_stream<R: Runtime>(
    app_handle: AppHandle<R>,
    url: &str,
    cookie: &str,
    prompt: &str,
) -> Result<String, String> {
    // 解析传入的 JSON 字符串
    let prompt_data: serde_json::Value = serde_json::from_str(prompt)
        .map_err(|e| format!("Failed to parse prompt JSON: {}", e))?;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(cookie)
            .map_err(|e| format!("Invalid header value: {}", e))?
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|err| format!("failed to generate client: {}", err))?;

    let response = client
        .post(url)
        .json(&prompt_data)  // 直接使用解析后的 JSON 数据
        .send()
        .await
        .map_err(|err| format!("failed to call API: {}", err))?;

    // 打印响应状态和头信息进行调试
    println!("Response status: {:?}", response.status());
    println!("Response headers: {:?}", response.headers());

    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                let chunk = from_utf8(&bytes).unwrap();
                // println!("{}", chunk);
                app_handle
                    .emit(
                        "fetch-stream-data",
                        StreamPayload {
                            message: chunk.to_string(),
                            prompt: prompt.to_string(),
                        },
                    )
                    .unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok("success".to_string())
}