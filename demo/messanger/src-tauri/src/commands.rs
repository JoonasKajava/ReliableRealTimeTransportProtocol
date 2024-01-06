use tauri::State;

use lib_rrttp::window::Window;

use crate::{AppState};

#[tauri::command]
pub fn bind(address: &str, state: State<AppState>) -> Result<String, String> {
    let test = Window::new(address).map_err(|e| {e.to_string()})?;
    state.0.lock().unwrap().window = Some(test.0);
    Ok(format!("Local socket has been bound to {}", address))
}