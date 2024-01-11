use tauri::State;
use typeshare::typeshare;

use lib_rrttp::application_layer::connector::Connector;
use lib_rrttp::application_layer::message_type::MessageType;

use crate::AppState;

#[typeshare]
pub struct Message {
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>, window: tauri::Window) -> Result<String, String> {
    let mut new_connector = Connector::new(address).map_err(|e| { e.to_string() })?;
    std::thread::spawn(move || {
        loop {
            let message = new_connector.1.recv().unwrap();
            window.emit("message", format!("{:?}", message)).unwrap();
        }
    });
    let guard = state.0.lock().unwrap();
    guard.connector = Some(new_connector.0);

    Ok(format!("Local socket has been bound to {}", address))
}

#[tauri::command]
pub fn connect(address: &str, state: State<AppStateInner>) -> Result<String, String> {
    /*    let guard = state.window_state.lock().unwrap();
        return match &guard.window {
            None => Err("Local socket has not been bound yet".to_string()),
            Some(window) => {
                match window.connect(address) {
                    Ok(_) => Ok(format!("Connected to remote: {}", address)),
                    Err(_) => Err("Failed to connect".to_string()),
                }
            }
        };*/
    Ok(format!("Connected to remote: {}", address))
}

#[tauri::command]
pub fn send_message(message: &str, state: State<AppStateInner>) -> Result<String, String> {
    /*    let mut guard = state.window_state.lock().unwrap();
        return match &mut guard.window {
            None => Err("Local socket has not been bound yet".to_string()),
            Some(window) => {
                match window.send(message.as_bytes()) {
                    Ok(_) => Ok(format!("Send message: {}", message)),
                    Err(e) => Err(format!("Failed to send message: {}", e)),
                }
            }
        };*/
    Ok(format!("Connected to remote: "))
}


#[tauri::command]
pub fn send_file(file_path: &str, state: State<AppStateInner>) -> Result<String, String> {
    /*    let mut guard = state.window_state.lock().unwrap();
        return match &mut guard.window {
            None => Err("Local socket has not been bound yet".to_string()),
            Some(window) => {
                match window.send_file(file_path) {
                    Ok(_) => Ok(format!("Send file: {}", file_path)),
                    Err(e) => Err(format!("Failed to send file: {}", e)),
                }
            }
        };*/

    Ok(format!("Connected to remote: "))
}