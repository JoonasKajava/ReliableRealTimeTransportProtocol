// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::time::SystemTime;

use commands::{bind, connect, send_file, send_message};
use lib_rrttp::application_layer::connector::Connector;

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

struct AppStateInner {
    pub connector: Option<Connector<MessageType>>,
}

struct AppState(Mutex<AppStateInner>);

impl Default for AppStateInner {
    fn default() -> Self {
        Self {
            connector: None
        }
    }
}


fn main() {
    setup_logger().expect("Failed to setup logger");
    tauri::Builder::default()
        .manage(AppState(Mutex::new(AppStateInner::default())))
        .invoke_handler(tauri::generate_handler![bind, connect, send_message, send_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
