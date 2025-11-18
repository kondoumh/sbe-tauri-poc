use tauri_plugin_opener::OpenerExt;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use std::collections::HashMap;
use std::sync::Mutex;

// Tab management state
struct TabState {
    tabs: HashMap<String, String>, // tab_id -> url
}

impl Default for TabState {
    fn default() -> Self {
        Self {
            tabs: HashMap::new(),
        }
    }
}

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

// Tab management commands
#[tauri::command]
async fn create_tab_content(
    state: tauri::State<'_, Mutex<TabState>>, 
    tab_id: String, 
    url: String
) -> Result<(), String> {
    let mut tab_state = state.lock().map_err(|e| e.to_string())?;
    tab_state.tabs.insert(tab_id.clone(), url.clone());
    
    // For now, we'll just store the tab info
    // In a full implementation, we'd create the webview content here
    println!("Created tab {} with URL {}", tab_id, url);
    Ok(())
}

#[tauri::command]
async fn close_tab(
    state: tauri::State<'_, Mutex<TabState>>, 
    tab_id: String
) -> Result<(), String> {
    let mut tab_state = state.lock().map_err(|e| e.to_string())?;
    tab_state.tabs.remove(&tab_id);
    
    println!("Closed tab {}", tab_id);
    Ok(())
}

#[tauri::command]
async fn switch_to_tab(
    state: tauri::State<'_, Mutex<TabState>>, 
    tab_id: String
) -> Result<(), String> {
    let tab_state = state.lock().map_err(|e| e.to_string())?;
    
    if let Some(url) = tab_state.tabs.get(&tab_id) {
        println!("Switched to tab {} with URL {}", tab_id, url);
        Ok(())
    } else {
        Err(format!("Tab {} not found", tab_id))
    }
}

#[tauri::command]
async fn navigate_tab(
    state: tauri::State<'_, Mutex<TabState>>, 
    tab_id: String, 
    url: String
) -> Result<(), String> {
    let mut tab_state = state.lock().map_err(|e| e.to_string())?;
    tab_state.tabs.insert(tab_id.clone(), url.clone());
    
    println!("Navigated tab {} to URL {}", tab_id, url);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(TabState::default()))
        .invoke_handler(tauri::generate_handler![
            greet, 
            open_url, 
            create_webview_window,
            create_tab_content,
            close_tab,
            switch_to_tab,
            navigate_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
