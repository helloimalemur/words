use std::sync::Mutex;
use chrono::Local;
use tauri::State;
use crate::AppState;

#[tauri::command]
pub fn date() -> String {
    Local::now().timestamp().to_string()
}

#[tauri::command]
pub fn write_string(string: String, state: State<'_, Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    app_state.strings.push(string);
}
#[tauri::command]
pub fn printall(state: State<'_, Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    for i in app_state.strings.clone() {
        println!("{}", i)
    }
}
#[tauri::command]
pub fn remove_entry(index: usize, state: State<'_, Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    // app_state.strings.remove(index);
    println!("removed: {}", index)
}