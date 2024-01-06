// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;


use std::sync::Mutex;
use commands::bind;
use lib_rrttp::window::Window;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState(Mutex<RRTPStateMutex>);

struct RRTPStateMutex {
    window: Option<Window>,
}


fn main() {
    tauri::Builder::default()
        .manage(AppState (Mutex::new(RRTPStateMutex {
            window: None,
        })))
        .invoke_handler(tauri::generate_handler![bind])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
