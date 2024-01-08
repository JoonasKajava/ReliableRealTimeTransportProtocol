use std::sync::Arc;
use log::info;
use tauri::State;

use lib_rrttp::window::Window;

use crate::{AppState};

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>, window: tauri::Window) -> Result<String, String> {
    let created_window = Window::new(address).map_err(|e| { e.to_string() })?;
    let mut guard = state.window_state.lock().unwrap();

    let new_window = Arc::new(created_window.0);


    guard.window = Some(new_window.clone());

    Window::listen(new_window.clone());

    std::thread::spawn(move || {
        info!("Listening for messages");
        loop {
            let message = created_window.1.recv().unwrap();
            let string = String::from_utf8(message).unwrap();
            info!("Received message: {}", string);
            window.emit("message", string).unwrap();
        }
    }
    );

    Ok(format!("Local socket has been bound to {}", address))
}

#[tauri::command]
pub fn connect(address: &str, state: State<AppState>) -> Result<String, String> {
    let guard = state.window_state.lock().unwrap();
    return match &guard.window {
        None => Err("Local socket has not been bound yet".to_string()),
        Some(window) => {
            match window.connect(address) {
                Ok(_) => Ok(format!("Connected to remote: {}", address)),
                Err(_) => Err("Failed to connect".to_string()),
            }
        }
    };
}

#[tauri::command]
pub fn send_message(message: &str, state: State<AppState>) -> Result<String, String> {
    let mut guard = state.window_state.lock().unwrap();
    return match &mut guard.window {
        None => Err("Local socket has not been bound yet".to_string()),
        Some(window) => {
            match window.send(message.as_bytes()) {
                Ok(_) => Ok(format!("Send message: {}", message)),
                Err(e) => Err(format!("Failed to send message: {}", e)),
            }
        }
    };
}


#[tauri::command]
pub fn send_file(file_path: &str, state: State<AppState>) -> Result<String, String> {
    let mut guard = state.window_state.lock().unwrap();
    return match &mut guard.window {
        None => Err("Local socket has not been bound yet".to_string()),
        Some(window) => {
            match window.send_file(file_path) {
                Ok(_) => Ok(format!("Send file: {}", file_path)),
                Err(e) => Err(format!("Failed to send file: {}", e)),
            }
        }
    };
}