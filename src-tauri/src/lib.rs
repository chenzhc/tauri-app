#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]
use log::info;
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
        .plugin(tauri_plugin_serialplugin::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
          // 允许指定目录
          let scope = app.fs_scope();
          let _ = scope.allow_directory("C:\\opt\\upFiles\\temp", false);
          Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![
                greet,
                mycustomcmd::my_custom_command,
                mycustomcmd::read_file
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

