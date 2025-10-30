use std::sync::Arc;

use reqwest::cookie::Jar;

#[tauri::command]
async fn get(url: String, cookies: Option<String>) -> Result<String, String> {
    let url = url.parse().unwrap();
    let jar = Jar::default();
    if let Some(cookies) = cookies {
        jar.add_cookie_str(&cookies, &url);
    }

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build().unwrap();

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())
        .unwrap()
        .text()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
