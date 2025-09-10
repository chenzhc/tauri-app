#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use std::{io::Error, net::TcpStream, sync::Mutex};
use tauri::{
    Manager, State,
    ipc::Response,
    path::{BaseDirectory, PathResolver},
    webview,
};
use tauri_plugin_shell::ShellExt;
use mqtt_protocol_core::mqtt;
use mqtt_protocol_core::mqtt::prelude::*;
use std::io::{Cursor, Read, Write};

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
pub async fn increase_counter(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {
    let mut state = state.lock().unwrap();
    state.counter += 1;

    Ok(state.counter)
}

#[tauri::command]
pub fn hello(handle: tauri::AppHandle) -> String {
    let resouce_path = handle
        .path()
        .resolve("lang/de.json", BaseDirectory::Resource)
        .unwrap();

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
        Ok(CustomResponse {
            message: message,
            other_val: 42 + number,
        })
    } else {
        Err("No Result".into())
    }
}

#[tauri::command(rename_all = "snake_case")]
pub fn my_custom_command(invoke_message: String, app_handle: tauri::AppHandle) -> String {
    info!(
        "I was invoked from JavaScript, with this message: {}",
        invoke_message
    );

    // let shell = app_handle.shell();
    // let output = tauri::async_runtime::block_on(async move {
    //     shell
    //         .command("echo")
    //         .args(["Hello from Rust!"])
    //         .output()
    //         .await
    //         .unwrap()
    // });

    // if output.status.success() {
    //     info!("Result: {:?}", String::from_utf8(output.stdout));
    // } else {
    //     info!("Exit with code: {}", output.status.code().unwrap());
    // }

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
pub async fn my_webview_label(webview_window: tauri::WebviewWindow, app_handle: tauri::AppHandle) {
    info!("WebviewWindow: {}", webview_window.label());
    let platform = tauri_plugin_os::platform();
    info!("Platform: {}", platform);
}

#[tauri::command]
pub async fn connect_mqtt() {
    // // Create connection
    // let mut connection = 
    //     mqtt::Connection::<mqtt::role::Client>::new(mqtt::Version::V5_0);
    // // Connect to broker
    // let mut stream = TcpStream::connect("yun.njngzk.com:1883").unwrap();
    
    let mut client_id = String::from("windows_client_ngss_");
    client_id.push_str(uuid::Uuid::new_v4().to_string().as_str());
    info!("client_id = {}", client_id);


    // // Build CONNECT packet
    // let connect_packet: mqtt::packet::v5_0::Connect = mqtt::packet::v5_0::Connect::builder()
    //     .client_id(client_id)
    //     .unwrap()
    //     .build().unwrap();
    // let events = connection.checked_send(connect_packet);

    // handle_events(&mut stream, &mut connection, events).unwrap();
    //  // Publish a message
    // let publish_packet = mqtt::packet::v5_0::Publish::builder()
    //     .topic_name("/test/topic")
    //     .unwrap()
    //     .qos(mqtt::packet::Qos::AtLeastOnce)
    //     .payload(b"Hello, MQTT!")
    //     .packet_id(connection.acquire_packet_id().unwrap())
    //     .build().unwrap();

        
     let client = MqttClient::new(client_id.to_string());
    
    // Connect to your broker (or any MQTT broker)
    client.connect("tcp://yun.njngzk.com:1883").await?;
    
    // Subscribe with callback
    // client.subscribe("sensors/+/data", |msg| {
    //     println!("📧 {}: {}", msg.topic, String::from_utf8_lossy(&msg.payload));
    // }).await?;
    
    // Publish a message
    client.publish("/test/topic/data", b"Hello, MQTT!").await?;

}

fn handle_events(
    stream: &mut TcpStream,
    _connection: &mut mqtt::Connection<mqtt::role::Client>,
    events: Vec<mqtt::connection::Event>,
) -> Result<(), Box<dyn std::error::Error>> {
    for event in events {
        match event {
            mqtt::connection::Event::RequestSendPacket { packet, .. } => {
                let buffer = packet.to_continuous_buffer();
                stream.write_all(&buffer)?;
                let packet_type = packet.packet_type();
                println!("Sent packet: {packet_type}");
            }
            mqtt::connection::Event::NotifyPacketReceived(packet) => match packet {
                mqtt::packet::Packet::V5_0Connack(connack) => {
                    let reason_code = connack.reason_code();
                    println!("CONNACK received: {reason_code:?}");
                }
                mqtt::packet::Packet::V5_0Puback(puback) => {
                    let packet_id = puback.packet_id();
                    println!("PUBACK received for packet ID: {packet_id}");
                }
                mqtt::packet::Packet::V5_0Pubrec(pubrec) => {
                    let packet_id = pubrec.packet_id();
                    println!("PUBREC received for packet ID: {packet_id}");
                }
                mqtt::packet::Packet::V5_0Pubcomp(pubcomp) => {
                    let packet_id = pubcomp.packet_id();
                    println!("PUBCOMP received for packet ID: {packet_id}");
                }
                _ => {
                    let packet_type = packet.packet_type();
                    println!("Received packet: {packet_type}");
                }
            },
            mqtt::connection::Event::NotifyPacketIdReleased(packet_id) => {
                println!("Packet ID {packet_id} released");
            }
            mqtt::connection::Event::NotifyError(error) => {
                eprintln!("MQTT Error: {error:?}");
            }
            mqtt::connection::Event::RequestClose => {
                println!("Connection close requested");
                return Ok(());
            }
            mqtt::connection::Event::RequestTimerReset { kind, duration_ms } => {
                println!("Timer reset requested: {kind:?} for {duration_ms} ms");
            }
            mqtt::connection::Event::RequestTimerCancel(kind) => {
                println!("Timer cancel requested: {kind:?}");
            }
        }
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_test_01() {
        crate::init();
        let _ = connect_mqtt();

    }
}
