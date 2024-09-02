use std::collections::HashMap;
use std::sync::Mutex;
use chrono::Local;
use rand::Rng;
use tauri::{Error, Manager, State};
use crate::AppState;
use crate::calculations::calc::{run_calculations, CalcResults};
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
    let mut updates: HashMap<u32, CalcResults> = HashMap::new();

    for file in cloned {
        let ind = file.index;
        let calc = get_calculations(file.path);
        updates.insert(ind, calc);
    }

    for (ind, calc_res) in updates {
        for mut entry in app_state.files.files_container.iter_mut() {
            if entry.index.eq(&ind) {
                entry.word_count = calc_res.word_count as u64;
                entry.parac = calc_res.paragraph_count as u64;
                entry.ws = calc_res.white_space as u64;
                entry.first_mu = calc_res.first_most_used.clone();
                entry.second_mu = calc_res.second_most_used.clone();
                entry.third_mu = calc_res.third_most_used.clone();

            }
        }
    }
}

fn get_calculations(path: String) -> CalcResults {
    let contents = read_docx_contents_to_string(path);
    run_calculations(contents)
}