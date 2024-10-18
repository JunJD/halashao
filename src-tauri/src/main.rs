// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// 在文件顶部添加这行
mod xhs;
mod config;
mod tools;

fn main() {
    tools::utils::init_logging_config();
    
    tauri::Builder::default()
        .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
