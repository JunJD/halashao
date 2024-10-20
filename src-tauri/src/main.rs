// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod xhs;
mod config;
mod tools;

use headless_chrome::{Browser, LaunchOptionsBuilder};
use log::info;
use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use anyhow::Result;

#[tauri::command]
async fn run_crawler(state: tauri::State<'_, Arc<Mutex<Option<Browser>>>>) -> Result<String, String> {
    let mut browser_option = state.lock().await;
    
    if browser_option.is_none() {
        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .headless(false)
                .idle_browser_timeout(Duration::from_secs(300))
                .build().map_err(|e| e.to_string())?
        ).map_err(|e| e.to_string())?;
        *browser_option = Some(browser);
    }

    let browser = browser_option.as_ref().unwrap();
    let tab = browser.new_tab().map_err(|e| format!("Failed to create new tab: {}", e))?;
    tab.navigate_to("https://www.xiaohongshu.com").map_err(|e| e.to_string())?;
    
    let mut login = xhs::login::XiaoHongShuLogin::new(
        unsafe { config::base_config::LOGIN_TYPE.to_owned() },
        tab.clone()  // 在这里克隆 tab
    );
    login.begin().await.map_err(|e| format!("Failed to begin login: {}", e))?;

    info!("login success");

    // 这里可以添加后续操作

    Ok("Crawler run successfully".to_string())
}

fn main() {
    crate::tools::utils::init_logging_config();
    let browser_state = Arc::new(Mutex::new(None::<Browser>));

    tauri::Builder::default()
        .manage(browser_state)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window_map = app.webview_windows();
                let window = window_map.get("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run_crawler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
