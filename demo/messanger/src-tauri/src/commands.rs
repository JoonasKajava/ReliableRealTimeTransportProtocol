use std::fs;
use std::path::Path;

use tauri::State;

use lib_rrttp::application_layer::connector::Connector;
use lib_rrttp::application_layer::message::Message;

use crate::AppState;
use crate::models::message_type::MessageType;
use crate::models::network_file_info::NetworkFileInfo;

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>, window: tauri::Window) -> Result<String, String> {
    let mut new_connector = Connector::new(address).map_err(|e| { e.to_string() })?;
    std::thread::spawn(move || {
        loop {
            let message = new_connector.1.recv().unwrap();
            window.emit("message", format!("{:?}", message)).unwrap();
        }
    });
    let mut guard = state.0.lock().unwrap();
    guard.connector = Some(new_connector.0);

    Ok(format!("Local socket has been bound to {}", address))
}

#[tauri::command]
pub fn connect(address: &str, state: State<AppState>) -> Result<String, String> {
    let app_state_lock = state.0.lock().unwrap();
    let connector = &app_state_lock.connector;
    let result = connector.as_ref().ok_or("Connector has not been bound")?.connect(address);
    match result {
        Ok(_) => Ok(format!("Connected to remote: {}", address)),
        Err(e) => Err(format!("Failed to connect to remote: {}", e)),
    }
}

#[tauri::command]
pub fn send_message(message: &str, state: State<AppState>) -> Result<String, String> {
    let mut guard = state.0.lock().unwrap();
    return match &mut guard.connector {
        None => Err("Local socket has not been bound yet".to_string()),
        Some(connector) => {
            let payload = Message {
                message_type: MessageType::Message,
                payload: message.as_bytes().to_vec(),
            };
            match connector.send(payload) {
                Ok(_) => Ok(format!("Send message: {}", message)),
                Err(e) => Err(format!("Failed to send message: {}", e)),
            }
        }
    };
}

#[tauri::command]
pub fn send_file_info(file_path: &str, state: State<AppState>) -> Result<String, String> {
    let file = fs::read(file_path).map_err(|e| { e.to_string() })?;

    let file_name = Path::new(file_path).file_name().ok_or_else(|| "Unable to read filename")?.to_str().ok_or_else(|| "Unable to read filename")?.to_string();
    let file_kind = infer::get(&file);

    let file_info = NetworkFileInfo {
        file_name,
        file_mime: file_kind.map(|e| e.mime_type().to_string()),
        file_size_in_bytes: file.len() as u32,
    };
    let bin = bincode::serialize(&file_info).map_err(|e| { e.to_string() })?;

    let message = Message {
        message_type: MessageType::FileInfo,
        payload: bin,
    };

    let app_state_guard = state.0.lock().unwrap();
    let guard = app_state_guard.connector.as_ref().unwrap();
    return match guard.send(message) {
        Ok(_) => Ok(format!("Send file info: {}", file_path)),
        Err(_) => Err(format!("Failed to send file info: {}", file_path)),
    };
}

#[tauri::command]
pub fn send_file(file_path: &str, state: State<AppState>) -> Result<String, String> {
    /*        let mut guard = state.0.lock().unwrap();
            return match &mut guard.connector {
                None => Err("Local socket has not been bound yet".to_string()),
                Some(connector) => {
                    match connector.send_file(file_path) {
                        Ok(_) => Ok(format!("Send file: {}", file_path)),
                        Err(e) => Err(format!("Failed to send file: {}", e)),
                    }
                }
            };*/

    Ok(format!("Connected to remote: "))
}