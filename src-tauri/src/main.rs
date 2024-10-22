// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod xhs;
mod config;
mod tools;

use headless_chrome::{Browser, LaunchOptionsBuilder};
use log::info;
use tauri::{Emitter, Manager, Window};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use std::time::Duration;
use anyhow::Result;

#[tauri::command]
async fn run_crawler(state: tauri::State<'_, Arc<Mutex<Option<Browser>>>>, window: Window) -> Result<String, String> {
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
    tab.set_user_agent(config::base_config::USER_AGENT.to_owned().as_str(), None, None);
    tab.navigate_to("https://www.xiaohongshu.com").map_err(|e| e.to_string())?;

    let mut login = xhs::login::XiaoHongShuLogin::new(
        unsafe { config::base_config::LOGIN_TYPE.to_owned() },
        tab.clone(),
        // Some("13156626720".to_string()),
        // config::base_config::COOKIES.to_owned(),
    );

    let (qrcode, mut rx) = login.begin().await.map_err(|e| e.to_string())?;

    // 返回二维码给前端
    tauri::async_runtime::spawn(async move {
        if let Some(login_success) = rx.recv().await {
            if login_success {
                // 登录成功时发送 qr_code_close 事件
                window.emit("qr_code_close", "Login successful").unwrap();
                info!("登录成功");
                let tab_clone = tab.clone();
                let cookies_result = tab_clone.get_cookies();
                let cookies_option = cookies_result.ok(); // 将 Result 转换为 Option
                let (cookie_str, cookie_dict) = tools::crawler_util::convert_cookies(cookies_option);
                let mut client = xhs::client::XiaoHongShuClient::new(
                    tab_clone,
                    HashMap::from([
                        ("User-Agent".to_owned(), config::base_config::USER_AGENT.to_owned()),
                        ("Cookie".to_owned(), cookie_str.to_owned()),
                        ("Origin".to_owned(), "https://www.xiaohongshu.com".to_owned()),
                        ("Referer".to_owned(), "https://www.xiaohongshu.com".to_owned()),
                        ("Content-Type".to_owned(), "application/json;charset=UTF-8".to_owned()),
                    ]),
                    cookie_dict.clone(),
                    None,
                    None
                );
                client.pong().await.unwrap();
            } else {
                info!("登录失败");
            }
        } else {
            info!("接收到空值或接收错误");
        }
    });

    Ok(qrcode)
}

fn main() {
    crate::tools::utils::init_logging_config();
    let browser_state = Arc::new(Mutex::new(None::<Browser>));

    tauri::Builder::default()
        .manage(browser_state)
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![run_crawler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
