// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use commands::{bind, connect, send_message, send_file};
use lib_rrttp::window::Window;

mod commands;

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

struct AppState {
    pub window_state: Mutex<RRTPStateMutex>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            window_state: Mutex::new(RRTPStateMutex {
                window: None,
            }),
        }
    }
}

struct RRTPStateMutex {
    window: Option<Arc<Window>>,
}


fn main() {
    setup_logger().expect("Failed to setup logger");
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![bind, connect, send_message, send_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
