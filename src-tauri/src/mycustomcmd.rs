#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use tauri::ipc::Response;

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

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_test_01() {
        crate::init();


    }
}
