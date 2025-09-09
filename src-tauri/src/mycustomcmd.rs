#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::Error, sync::Mutex};
use log::info;
use tauri::{ipc::Response, path::{BaseDirectory, PathResolver}, webview, Manager};

pub struct Database;

#[derive(serde::Serialize)]
pub struct CustomResponse {
    message: String,
    other_val: usize,
}

pub struct AppData {
    pub welcome_message: &'static str,
}

#[derive(Default)]
pub struct AppState {
    pub counter: u32,
}

pub async fn some_other_function() -> Option<String> {
    Some("response".into())
}

#[tauri::command]
pub fn get_app_data(app: tauri::AppHandle) -> String {
    let data = app.state::<AppData>().welcome_message;
    return data.to_string();
}

#[tauri::command]
pub fn get_app_state(app: tauri::AppHandle) -> u32 {
    let state = app.state::<Mutex<AppState>>();

    let mut state = state.lock().unwrap();
    state.counter += 1;

    return state.counter;
}

#[tauri::command]
pub fn hello(handle: tauri::AppHandle) -> String {
    let resouce_path = handle.path()
        .resolve("lang/de.json", BaseDirectory::Resource).unwrap();

    let file = std::fs::File::open(&resouce_path).unwrap();
    let lang_de: serde_json::Value = serde_json::from_reader(file).unwrap();

    lang_de.get("hello").unwrap().to_string()
}

#[tauri::command]
pub async fn my_custom_command2(
    window: tauri::Window,
    number: usize,
    database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
    info!("Called from {}", window.label());
    let result = some_other_function().await;
    if let Some(message) = result {
        Ok(
            CustomResponse { message: message, other_val: 42 + number }
        )
    } else {
        Err("No Result".into())
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn my_custom_command(invoke_message: String) -> String {
    info!("I was invoked from JavaScript, with this message: {}", invoke_message);
    return "Hello from Rust!".into();
}

#[tauri::command]
pub fn read_file() -> Response {
    let data = std::fs::read("C:/opt/upFiles/temp").unwrap();
    return tauri::ipc::Response::new(data);
}


#[tauri::command]
pub fn login(user: String, password: String) -> Result<String, String> {
    if user == "tauri" && password == "tauri" {
        Ok("logged_in".to_string())
    } else {
        Err("invalid credentials".to_string())
    }
}

#[tauri::command]
pub async fn my_webview_label(webview_window: tauri::WebviewWindow) {
    info!("WebviewWindow: {}", webview_window.label());

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_test_01() {
        crate::init();


    }
}
