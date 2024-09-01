// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod entries;
use entries::*;
mod files;
use files::*;
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




fn main() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
          app.manage(Mutex::new(AppState::default()));
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            // window.close_devtools();
            Ok(())
        })
        .invoke_handler(
          tauri::generate_handler![date, write_string, printall, remove_entry, open_file]
        )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
