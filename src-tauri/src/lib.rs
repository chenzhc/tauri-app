#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Mutex;
use log::info;
use tauri::{Listener, Manager};
use tauri_plugin_fs::FsExt;

pub mod mycustomcmd;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(mycustomcmd::Database{})
        .setup(|app| {
            app.manage(mycustomcmd::AppData {
                welcome_message: "Welcome to Tauri!",
            });
            app.manage(Mutex::new(mycustomcmd::AppState::default()));
            app.once("ready", |event| {
                info!("app is ready!");
            });
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                // let window = app.get_window("main").unwrap();
                // window.open_devtools();
                // window.close_devtools();
            }
          Ok(())
        })
        .plugin(tauri_plugin_serialplugin::init())
        .plugin(tauri_plugin_fs::init())        
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet,
                mycustomcmd::my_custom_command,
                mycustomcmd::read_file,
                mycustomcmd::login,
                mycustomcmd::my_webview_label,
                mycustomcmd::my_custom_command2,
                mycustomcmd::hello,
                mycustomcmd::get_app_data,
                mycustomcmd::get_app_state
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// init log config 
pub fn init() {
    let _ = env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .is_test(true)
        .try_init();
}

