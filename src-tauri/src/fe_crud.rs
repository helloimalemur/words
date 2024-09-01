use std::sync::Mutex;
use chrono::Local;
use tauri::State;
use crate::AppState;
use crate::files::DocFile;

#[tauri::command]
pub fn date() -> String {
    Local::now().timestamp().to_string()
}


#[tauri::command]
pub fn printall(state: State<'_, Mutex<AppState>>) {
    // let mut app_state = state.lock().unwrap();
    let files = state.lock().unwrap().files.files_container.clone();
    for i in files.iter() {
        println!("{:?}", i)
    }
}

#[tauri::command]
pub fn get_entries(state: State<'_, Mutex<AppState>>) -> String {
    // let mut app_state = state.lock().unwrap();
    let files = state.lock().unwrap().files.files_container.clone();
    serde_json::to_string(&files).unwrap_or_default()
}

#[tauri::command]
pub fn remove_entry(index: usize, state: State<'_, Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    // app_state.strings.remove(index);
    println!("removed: {}", index)
}