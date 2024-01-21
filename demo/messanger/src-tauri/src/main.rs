// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::SystemTime;

use log::error;
use tauri::Manager;

use commands::{bind, connect, send_file, send_file_info, send_message};
use lib_rrttp::application_layer::connector::Connector;

use crate::models::log_message::LogSuccessMessage;
use crate::models::message_type::MessageType;

mod commands;
mod models;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct ConnectorState {
    pub connector: Option<Connector<MessageType>>,
}

struct AppState {
    pub connector_state: Mutex<ConnectorState>,
    pub log_sender: Sender<LogSuccessMessage>,
}

impl AppState {
    fn new(log_sender: Sender<LogSuccessMessage>) -> Self {
        Self {
            connector_state: Default::default(),
            log_sender,
        }
    }
}

impl Default for ConnectorState {
    fn default() -> Self {
        Self {
            connector: None
        }
    }
}


fn main() {
    setup_logger().expect("Failed to setup logger");
    tauri::Builder::default()
        .setup(|app| {
            let (log_sender, log_receiver): (Sender<LogSuccessMessage>, Receiver<LogSuccessMessage>) = std::sync::mpsc::channel();
            let app_state = AppState::new(log_sender);
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    let message = log_receiver.recv().unwrap();
                    match handle.emit_all("log", message) {
                        Ok(_) => {}
                        Err(e) => error!("Failed to emit log message: {}", e)
                    }
                }
            });
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![bind, connect, send_message, send_file, send_file_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
