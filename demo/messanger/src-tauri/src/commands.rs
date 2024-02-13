use std::fs;
use std::path::Path;

use tauri::AppHandle;
use tauri::{Manager, State};

use lib_rrttp::application_layer::connection_manager::{
    ConnectionManager, ConnectionManagerInterface,
};

use crate::connection_processor::ConnectionProcessor;
use crate::message::Message;
use crate::models::log_message::{LogErrorMessage, LogMessageResult, LogSuccessMessage};
use crate::models::network_file_info::NetworkFileInfo;
use crate::AppState;

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>) -> LogMessageResult {
    let ConnectionManagerInterface {
        connection_manager,
        connection_events,
        message_sender,
    } = ConnectionManager::start(address)
        .map_err(|e| LogErrorMessage::LocalSocketBindFailed(e.to_string()))?;

    let sender = state.log_sender.clone();

    let processor = ConnectionProcessor::new(sender.clone(), state.message_state.clone());

    std::thread::spawn(move || loop {
        match connection_events.recv() {
            Ok(connection_event) => {
                if let Err(error) = processor.process_connection_event(connection_event) {
                    let _ = sender.send(LogSuccessMessage::Error(error.to_string()));
                }
            }
            Err(_) => {
                let _ = sender.send(LogSuccessMessage::Error(
                    "Connection event receiver has been dropped".to_string(),
                ));
                break;
            }
        }
    });
    let mut guard = state.connector_state.lock().unwrap();
    guard.set_message_sender(message_sender);
    guard.connector = Some(connection_manager);

    Ok(LogSuccessMessage::LocalSocketBindSuccess(
        address.to_string(),
    ))
}

#[tauri::command]
pub fn connect(address: &str, state: State<AppState>) -> LogMessageResult {
    let app_state_lock = state.connector_state.lock().unwrap();
    let connector = &app_state_lock.connector;
    let result = connector
        .as_ref()
        .ok_or(LogErrorMessage::LocalSocketNotBound)?
        .connect(address);
    match result {
        Ok(_) => Ok(LogSuccessMessage::ConnectedToRemote(address.to_string())),
        Err(e) => Err(LogErrorMessage::ConnectionError(e.to_string())),
    }
}

#[tauri::command]
pub fn send_message(message: &str, state: State<AppState>) -> LogMessageResult {
    let mut guard = state.connector_state.lock().unwrap();
    return match &mut guard.connector {
        None => Err(LogErrorMessage::LocalSocketNotBound),
        Some(connector) => {
            let message_for_network = Message::String(message.to_string());
            let payload = message_for_network.try_into().unwrap();
            match connector.send(payload) {
                Ok(_) => Ok(LogSuccessMessage::MessageSent(message.to_string())),
                Err(e) => Err(LogErrorMessage::MessageSendError(e.to_string())),
            }
        }
    };
}

#[tauri::command]
pub fn send_file_info(file_path: &str, state: State<AppState>) -> LogMessageResult {
    let file = fs::read(file_path).map_err(|e| LogErrorMessage::FileSendError(e.to_string()))?;

    let file_name_read_error =
        || LogErrorMessage::FileSendError("Unable to read filename".to_string());
    let file_name = Path::new(file_path)
        .file_name()
        .ok_or_else(file_name_read_error)?
        .to_str()
        .ok_or_else(file_name_read_error)?
        .to_string();
    let file_kind = infer::get(&file);

    let file_info = NetworkFileInfo {
        file_name,
        file_mime: file_kind.map(|e| e.mime_type().to_string()),
        file_size_in_bytes: file.len() as u32,
    };

    let file_info_clone = file_info.clone();

    let message = Message::FileInfo(file_info);

    let app_state_guard = state.connector_state.lock().unwrap();
    let guard = app_state_guard.connector.as_ref().unwrap();
    let payload = message
        .try_into()
        .map_err(|e| LogErrorMessage::FileSendError(e))?;
    return match guard.send(payload) {
        Ok(_) => {
            let mut guard = state.message_state.lock().unwrap();
            guard.file_to_send.replace(file_path.to_string());
            Ok(LogSuccessMessage::FileInfoSent(file_info_clone))
        }
        Err(e) => Err(LogErrorMessage::FileSendError(e.to_string())),
    };
}

#[tauri::command]
pub fn respond_to_file_info(ready: bool, file: &str, state: State<AppState>) -> LogMessageResult {
    let connector_guard = state.connector_state.lock().unwrap();
    let guard = connector_guard.connector.as_ref().unwrap();
    if ready {
        if Path::new(file).exists() {
            return Err(LogErrorMessage::InvalidFileResponse(
                "File already exists".to_string(),
            ));
        }

        state
            .message_state
            .lock()
            .unwrap()
            .path_to_write_new_file
            .replace(file.to_string());
    }
    let message = Message::ResponseToFileInfo { accepted: ready };
    let payload = message
        .try_into()
        .map_err(|e| LogErrorMessage::InvalidFileResponse(e))?;

    return match guard.send(payload) {
        Ok(_) => Ok(LogSuccessMessage::FileResponseSent),
        Err(e) => Err(LogErrorMessage::InvalidFileResponse(e.to_string())),
    };
}

pub fn send_file(file_path: &str, app_handle: &AppHandle) -> Result<String, String> {
    let state = app_handle.state::<AppState>();
    let mut guard = state.connector_state.lock().unwrap();
    return match &mut guard.connector {
        None => Err("Local socket has not been bound yet".to_string()),
        Some(connector) => {
            let message = Message::FileData(fs::read(file_path).map_err(|e| e.to_string())?);
            let payload = message
                .try_into()
                .map_err(|e| format!("Failed to send file: {}", e))?;
            match connector.send(payload) {
                Ok(_) => Ok(format!("Send file: {}", file_path)),
                Err(e) => Err(format!("Failed to send file: {}", e)),
            }
        }
    };
}
