use std::collections::HashMap;
use std::sync::Mutex;
use chrono::Local;
use rand::Rng;
use tauri::{Error, Manager, State};
use crate::AppState;
use crate::docx::loader::read_docx_contents_to_string;
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
pub fn remove_entry(index: u32, state: State<'_, Mutex<AppState>>) {
    let mut bind = state.lock();
    let mut app_state = bind.as_mut().unwrap();
    app_state.files.remove(index);
}

#[tauri::command]
pub fn remove_all_entries(state: State<'_, Mutex<AppState>>) {
    let mut bind = state.lock();
    let mut app_state = bind.as_mut().unwrap();
    app_state.files.files_container.clear();
}

#[tauri::command]
pub fn update_word_count(state: State<'_, Mutex<AppState>>) {
    let mut bind = state.lock();
    let mut app_state = bind.as_mut().unwrap();
    let cloned = app_state.files.files_container.clone();
    let mut updates: HashMap<u32, u8> = HashMap::new();

    for file in cloned {
        let ind = file.index;
        let wc = get_word_count(file.path);
        updates.insert(ind, wc);
    }

    for (ind, wc) in updates {
        for mut entry in app_state.files.files_container.iter_mut() {
            if entry.index.eq(&ind) {
                entry.word_count = wc as u64
            }
        }
    }
}

fn get_word_count(path: String) -> String {
    read_docx_contents_to_string(path)
}