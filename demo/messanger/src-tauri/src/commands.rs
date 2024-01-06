use log::info;
use tauri::State;

use lib_rrttp::window::Window;

use crate::{AppState};

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>, window: tauri::Window) -> Result<String, String> {
    let created_window = Window::new(address).map_err(|e| { e.to_string() })?;
    let mut guard = state.window_state.lock().unwrap();
    guard.window = Some(created_window.0);

    std::thread::spawn(move || {
        info!("Listening for messages");
        loop {
            let message = created_window.1.recv().unwrap();
            let string = String::from_utf8(message).unwrap();
            window.emit("message", string).unwrap();
        }
    }
    );

    Ok(format!("Local socket has been bound to {}", address))
}