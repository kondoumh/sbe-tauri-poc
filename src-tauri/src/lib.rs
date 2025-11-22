use tauri_plugin_opener::OpenerExt;
use tauri::{WebviewUrl, WebviewWindowBuilder, Emitter, Manager};
use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use url::Url;
use tauri::webview::Cookie;

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
    println!("🚀 Creating WebView window: {} -> {}", label, url);
    let webview_url = WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?);
    
    let window = WebviewWindowBuilder::new(&app, &label, webview_url)
        .title("Scrapbox")
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .resizable(true)
        .visible(false)
        .initialization_script(&format!(
            r#"
            // Set window label for the external script
            window.navigationTrackerLabel = '{}';
            
            // Load and execute external navigation tracker script
            {}
            "#,
            label,
            include_str!("../scripts/navigation-tracker.js")
        ))
        .build()
        .map_err(|e| e.to_string())?;
    
    // Show window after it's fully initialized
    window.show().map_err(|e| e.to_string())?;
    
    Ok(())
}



// Command to track navigation from frontend
#[tauri::command]
async fn track_navigation(app: tauri::AppHandle, window_label: String, url: String, title: String) -> Result<(), String> {
    println!("Navigation tracked: {} -> {} ({})", window_label, url, title);
    
    // Emit event to main window for history tracking
    app.emit("add-to-recent", NavigationEvent {
        window_label,
        url,
        title,
    }).map_err(|e| e.to_string())?;
    
    Ok(())
}

// Command to update title of recent window
#[tauri::command]
async fn update_recent_title(app: tauri::AppHandle, window_label: String, url: String, title: String) -> Result<(), String> {
    println!("Title updated for {}: {} ({})", window_label, url, title);
    
    // Emit event to update recent window title
    app.emit("update-recent-title", NavigationEvent {
        window_label,
        url,
        title,
    }).map_err(|e| e.to_string())?;
    
    Ok(())
}

// Helper function to build cookie header from webview cookies
fn build_cookie_header(cookies: Vec<Cookie<'static>>) -> String {
    cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<_>>()
        .join("; ")
}

// Command to fetch Scrapbox pages with authentication (supports both public and private projects)
#[tauri::command]
async fn fetch_scrapbox_pages(
    app: tauri::AppHandle,
    project: String, 
    skip: Option<i32>, 
    limit: Option<i32>, 
    sort: Option<String>
) -> Result<ScrapboxPagesResponse, String> {
    let skip = skip.unwrap_or(0);
    let limit = limit.unwrap_or(20);
    let sort = sort.unwrap_or_else(|| "updated".to_string());
    
    let api_url = format!(
        "https://scrapbox.io/api/pages/{}?skip={}&limit={}&sort={}",
        project, skip, limit, sort
    );
    
    println!("🔍 Fetching private Scrapbox pages: {}", api_url);
    
    // Get cookies from any existing WebView that has accessed Scrapbox
    let scrapbox_url = Url::parse("https://scrapbox.io").map_err(|e| format!("Invalid URL: {}", e))?;
    
    // Try to get cookies from main window's webview first
    let cookies = if let Some(main_window) = app.get_webview_window("main") {
        main_window.cookies_for_url(scrapbox_url.clone())
            .map_err(|e| format!("Failed to get cookies: {}", e))?
    } else {
        // If no main window, try to find any webview window
        let webviews = app.webview_windows();
        if let Some((_, webview)) = webviews.iter().next() {
            webview.cookies_for_url(scrapbox_url.clone())
                .map_err(|e| format!("Failed to get cookies: {}", e))?
        } else {
            println!("⚠️ No webview windows found, proceeding without authentication");
            Vec::new()
        }
    };
    
    let client = reqwest::Client::new();
    let mut request_builder = client.get(&api_url);
    
    // Add cookies if available
    if !cookies.is_empty() {
        let cookie_header = build_cookie_header(cookies);
        println!("🍪 Using cookies for authentication: {} cookies", cookie_header.matches(';').count() + 1);
        request_builder = request_builder.header("Cookie", cookie_header);
    }
    
    let response = request_builder
        .send()
        .await
        .map_err(|e| format!("Failed to fetch pages: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {} - This might be a private project requiring authentication", response.status()));
    }
    
    let pages_data: ScrapboxPagesResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    println!("✅ Fetched {} pages from private project {}", pages_data.pages.len(), project);
    
    Ok(pages_data)
}

#[derive(serde::Serialize, Clone)]
struct NavigationEvent {
    window_label: String,
    url: String,
    title: String,
}

// Scrapbox API structures
#[derive(Serialize, Deserialize, Clone)]
struct ScrapboxPage {
    id: String,
    title: String,
    image: Option<String>,
    descriptions: Vec<String>,
    user: ScrapboxUser,
    #[serde(rename = "lastUpdateUser")]
    last_update_user: Option<ScrapboxUser>,
    pin: i64,
    views: i32,
    linked: i32,
    created: i64,
    updated: i64,
    accessed: Option<i64>,
    #[serde(rename = "linesCount")]
    lines_count: Option<i32>,
    #[serde(rename = "charsCount")]
    chars_count: Option<i32>,
    helpfeels: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ScrapboxUser {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct ScrapboxPagesResponse {
    #[serde(rename = "projectName")]
    project_name: String,
    skip: i32,
    limit: i32,
    count: i32,
    pages: Vec<ScrapboxPage>,
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
            navigate_tab,
            track_navigation,
            update_recent_title,
            fetch_scrapbox_pages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
