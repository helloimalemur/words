// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use chrono::Local;
use tauri::{Manager, State};

#[derive(serde::Deserialize)]
struct AppState {
    strings: Vec<String>
}

impl AppState {
    fn default() -> AppState {
        AppState { strings: vec![] }
    }
    fn print_strings(&mut self) {
        for i in self.strings.clone() {
            println!("{}", i)
        }
    }
    fn add_string<T: ToString>(&mut self, string: T) {
        self.strings.push(string.to_string())
    }
    fn remote_string(&mut self, string: String) {
        let mut pos = 0;
        for i in self.strings.clone() {
            if i.contains(&string) {
                self.strings.remove(pos);
            }
            pos += 1
        }
    }
}

#[tauri::command]
fn date() -> i64 {
    Local::now().timestamp()
}

#[tauri::command]
fn write_string(string: String, state: State<'_, Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    app_state.strings.push(string);
    for i in app_state.strings.clone() {
        println!("{}", i)
    }
}




fn main() {
    tauri::Builder::default()
        .setup(|app| {
          app.manage(Mutex::new(AppState::default()));
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            // window.close_devtools();
            Ok(())
        })
        .invoke_handler(
          tauri::generate_handler![date, write_string]
        )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
