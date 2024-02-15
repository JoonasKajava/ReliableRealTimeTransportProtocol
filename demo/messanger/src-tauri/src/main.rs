// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use log::{error, info};
use tauri::Manager;

use commands::{bind, connect, respond_to_file_info, send_file, send_file_info, send_message};
use lib_rrttp::application_layer::connection_manager::ConnectionManager;

use crate::models::file_models::FileInfo;
use crate::models::log_message::LogSuccessMessage;

mod commands;
mod connection_processor;
mod message;
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
    pub connector: Option<ConnectionManager>,
    message_sender: Option<SyncSender<Vec<u8>>>,
}

impl ConnectorState {
    pub fn set_message_sender(&mut self, sender: SyncSender<Vec<u8>>) {
        self.message_sender = Some(sender);
    }
    pub fn send_message(&self, message: impl Into<Vec<u8>>) {
        if let Some(sender) = &self.message_sender {
            let _ = sender.send(message.into());
        }
    }
}

#[derive(Default)]
struct MessageState {
    pub outgoing_file: Option<FileInfo>,
    pub incoming_file: Option<FileInfo>,
}

struct AppState {
    pub connector_state: Mutex<ConnectorState>,
    pub log_sender: Sender<LogSuccessMessage>,
    pub message_state: Arc<Mutex<MessageState>>,
}

impl AppState {
    fn new(log_sender: Sender<LogSuccessMessage>) -> Self {
        Self {
            connector_state: Default::default(),
            log_sender,
            message_state: Default::default(),
        }
    }
}

impl Default for ConnectorState {
    fn default() -> Self {
        Self {
            connector: None,
            message_sender: None,
        }
    }
}

fn main() {
    setup_logger().expect("Failed to setup logger");
    tauri::Builder::default()
        .setup(|app| {
            let (log_sender, log_receiver): (
                Sender<LogSuccessMessage>,
                Receiver<LogSuccessMessage>,
            ) = std::sync::mpsc::channel();
            let app_state = AppState::new(log_sender);
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                info!("Starting log receiver");
                loop {
                    let message = log_receiver.recv().unwrap();
                    info!("Received log message: {:?}", message);
                    match handle.emit_all("log", message) {
                        Ok(_) => {}
                        Err(e) => error!("Failed to emit log message: {}", e),
                    }
                }
            });
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            bind,
            connect,
            send_message,
            send_file_info,
            respond_to_file_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
