use tauri_plugin_opener::OpenerExt;
use tauri::{WebviewUrl, WebviewWindowBuilder, Emitter};
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
    println!("🚀 Creating WebView window: {} -> {}", label, url);
    let webview_url = WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?);
    
    let window = WebviewWindowBuilder::new(&app, &label, webview_url)
        .title("Scrapbox")
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .resizable(true)
        .visible(false)
        .initialization_script(&format!(r#"
            console.log('🔧 Initializing navigation tracker for: {}');
            
            // Wait for Tauri API to be available
            function waitForTauriAPI(callback, maxAttempts = 20) {{
                let attempts = 0;
                
                function checkAPI() {{
                    attempts++;
                    console.log('🔍 Checking Tauri API availability (attempt ' + attempts + ')');
                    
                    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {{
                        console.log('✅ Tauri API is available!');
                        callback();
                    }} else if (attempts < maxAttempts) {{
                        console.log('⏳ Tauri API not ready, retrying in 200ms...');
                        setTimeout(checkAPI, 200);
                    }} else {{
                        console.error('❌ Tauri API failed to load after ' + maxAttempts + ' attempts');
                        console.log('Debug info:', {{
                            __TAURI__: !!window.__TAURI__,
                            core: window.__TAURI__ ? !!window.__TAURI__.core : false,
                            invoke: window.__TAURI__ && window.__TAURI__.core ? !!window.__TAURI__.core.invoke : false
                        }});
                    }}
                }}
                
                checkAPI();
            }}
            
            // Navigation tracking state
            let currentUrl = window.location.href;
            let currentTitle = document.title || window.location.hostname || 'Untitled';
            
            // Function to track navigation
            function trackNavigation(url, title, source) {{
                console.log('🔄 Navigation tracked (' + source + '):', title, '→', url);
                
                if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {{
                    console.log('📡 Sending to Rust:', {{ windowLabel: '{}', url: url, title: title }});
                    window.__TAURI__.core.invoke('track_navigation', {{
                        windowLabel: '{}',
                        url: url,
                        title: title
                    }}).then(result => {{
                        console.log('✅ Track navigation success:', result);
                    }}).catch(err => {{
                        console.error('❌ Failed to track navigation:', err);
                    }});
                }} else {{
                    console.error('❌ Tauri API not available when trying to track navigation');
                }}
            }}
            
            // Initialize tracking when Tauri API is ready
            waitForTauriAPI(function() {{
                console.log('🚀 Starting navigation tracking...');
                
                // Track initial page load
                trackNavigation(currentUrl, currentTitle, 'initialization');
                
                // Start monitoring for changes
                setInterval(() => {{
                    const newUrl = window.location.href;
                    const newTitle = document.title || window.location.hostname || 'Untitled';
                    
                    if (newUrl !== currentUrl || newTitle !== currentTitle) {{
                        trackNavigation(newUrl, newTitle, 'polling');
                        currentUrl = newUrl;
                        currentTitle = newTitle;
                    }}
                }}, 300);
                
                // Listen for browser navigation events
                window.addEventListener('popstate', () => {{
                    setTimeout(() => {{
                        const url = window.location.href;
                        const title = document.title || window.location.hostname || 'Untitled';
                        trackNavigation(url, title, 'popstate');
                        currentUrl = url;
                        currentTitle = title;
                    }}, 50);
                }});
                
                window.addEventListener('hashchange', () => {{
                    const url = window.location.href;
                    const title = document.title || window.location.hostname || 'Untitled';
                    trackNavigation(url, title, 'hashchange');
                    currentUrl = url;
                    currentTitle = title;
                }});
            }});
        "#, label, label, label))
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

#[derive(serde::Serialize, Clone)]
struct NavigationEvent {
    window_label: String,
    url: String,
    title: String,
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
            update_recent_title
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
