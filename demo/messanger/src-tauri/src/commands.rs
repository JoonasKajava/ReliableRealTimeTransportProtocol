use std::fs;
use std::path::Path;

use tauri::State;

use lib_rrttp::application_layer::connector::Connector;
use lib_rrttp::application_layer::message::Message;

use crate::AppState;
use crate::models::log_message::{LogErrorMessage, LogMessageResult, LogSuccessMessage};
use crate::models::message_type::MessageType;
use crate::models::network_file_info::NetworkFileInfo;

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>) -> LogMessageResult {
    let mut new_connector = Connector::new(address).map_err(|e| LogErrorMessage::LocalSocketBindFailed(e.to_string()))?;

    let sender = state.log_sender.clone();
    std::thread::spawn(move || {
        loop {
            let message = new_connector.1.recv().unwrap();
            sender.send(message.into()).unwrap()
        }
    });
    let mut guard = state.connector_state.lock().unwrap();
    guard.connector = Some(new_connector.0);

    Ok(LogSuccessMessage::LocalSocketBindSuccess(address.to_string()))
}

#[tauri::command]
pub fn connect(address: &str, state: State<AppState>) -> LogMessageResult {
    let app_state_lock = state.connector_state.lock().unwrap();
    let connector = &app_state_lock.connector;
    let result = connector.as_ref().ok_or(LogErrorMessage::LocalSocketNotBound)?.connect(address);
    match result {
        Ok(_) => Ok(LogSuccessMessage::ConnectedToRemote(address.to_string())),
        Err(e) => Err(LogErrorMessage::ConnectionError(e.to_string()))
    }
}

#[tauri::command]
pub fn send_message(message: &str, state: State<AppState>) -> LogMessageResult {
    let mut guard = state.connector_state.lock().unwrap();
    return match &mut guard.connector {
        None => Err(LogErrorMessage::LocalSocketNotBound),
        Some(connector) => {
            let payload = Message {
                message_type: MessageType::Message,
                payload: message.as_bytes().to_vec(),
            };
            match connector.send(payload) {
                Ok(_) => Ok(LogSuccessMessage::MessageSent(message.to_string())),
                Err(e) => Err(LogErrorMessage::MessageSendError(e.to_string()))
            }
        }
    };
}

#[tauri::command]
pub fn send_file_info(file_path: &str, state: State<AppState>) -> LogMessageResult {
    let file = fs::read(file_path).map_err(|e| LogErrorMessage::FileSendError(e.to_string()))?;

    let file_name_read_error = || LogErrorMessage::FileSendError("Unable to read filename".to_string());
    let file_name = Path::new(file_path).file_name().ok_or_else(file_name_read_error)?.to_str().ok_or_else(file_name_read_error)?.to_string();
    let file_kind = infer::get(&file);

    let file_info = NetworkFileInfo {
        file_name,
        file_mime: file_kind.map(|e| e.mime_type().to_string()),
        file_size_in_bytes: file.len() as u32,
    };

    let file_info_clone = file_info.clone();
    let bin: Result<Vec<u8>, String> = file_info.try_into();

    let message = Message {
        message_type: MessageType::FileInfo,
        payload: bin.map_err(|e| LogErrorMessage::FileSendError(e))?,
    };

    let app_state_guard = state.connector_state.lock().unwrap();
    let guard = app_state_guard.connector.as_ref().unwrap();
    return match guard.send(message) {
        Ok(_) => Ok(LogSuccessMessage::FileInfoSent(file_info_clone)),
        Err(e) => Err(LogErrorMessage::FileSendError(e.to_string()))
    };
}

#[tauri::command]
pub fn respond_to_file_info(ready: bool, file: &str, state: State<AppState>) -> LogMessageResult {
    let connector_guard = state.connector_state.lock().unwrap();
    let guard = connector_guard.connector.as_ref().unwrap();
    if ready {
        if Path::new(file).exists() {
            return Err(LogErrorMessage::InvalidFileResponse("File already exists".to_string()));
        }

        state.path_to_write_new_file.lock().unwrap().replace(file.to_string());
    }

    let message = Message {
        message_type: match ready {
            true => MessageType::FileAccepted,
            false => MessageType::FileRejected,
        },
        payload: vec![],
    };
    return match guard.send(message) {
        Ok(_) => Ok(LogSuccessMessage::FileResponseSent),
        Err(e) => Err(LogErrorMessage::InvalidFileResponse(e.to_string()))
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