#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use std::sync::Mutex;
use tauri::{
    Listener, Manager,
    menu::{Menu, MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
};
use tauri_plugin_fs::FsExt;

pub mod mycustomcmd;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(mycustomcmd::Database {})
        .setup(|app| {
            app.manage(mycustomcmd::AppData {
                welcome_message: "Welcome to Tauri!",
            });
            app.manage(Mutex::new(mycustomcmd::AppState::default()));
            app.once("ready", |event| {
                info!("app is ready!");
            });

            let menu_list = MenuBuilder::new(app)
                .text("open", "打开")
                .text("close", "关闭")
                .build()?;

            app.set_menu(menu_list)?;
            app.on_menu_event(move |app_handle, event| {
                info!("menu event: {:?}", event.id());
                match event.id().0.as_str() {
                    "open" => {
                        info!("open event");
                    }
                    "close" => {
                        info!("close event");
                        app_handle.exit(0);
                    }
                    _ => {
                        info!("unexpected menu event");
                    }
                }
            });

            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        info!("quit menu item was clicked");
                        // app.exit(0);
                        // exits the app with the given status code
                        app.exit(0);

                        // restarts the app
                        // app.restart();
                    }
                    _ => {
                        info!("menu item {:?} not handled", event.id);
                    }
                })
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_serialplugin::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            mycustomcmd::my_custom_command,
            mycustomcmd::read_file,
            mycustomcmd::login,
            mycustomcmd::my_webview_label,
            mycustomcmd::my_custom_command2,
            mycustomcmd::hello,
            mycustomcmd::get_app_data,
            mycustomcmd::get_app_state,
            mycustomcmd::increase_counter,
            mycustomcmd::connect_mqtt
        ]);

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }

    builder
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
