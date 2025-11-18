use tauri_plugin_opener::OpenerExt;
use tauri::{WebviewUrl, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_webview_window(app: tauri::AppHandle, url: String, label: String) -> Result<(), String> {
    let webview_url = WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?);
    
    WebviewWindowBuilder::new(&app, &label, webview_url)
        .title("Scrapbox")
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .resizable(true)
        .build()
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_url, create_webview_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
