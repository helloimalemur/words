// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fe_crud;
use fe_crud::*;
mod files;
use files::*;
use std::sync::{Arc, Mutex};
use chrono::Local;
use tauri::{Manager, State};

#[derive(serde::Deserialize)]
struct AppState {
    files: DocFiles
}

impl AppState {
    fn default() -> AppState {
        AppState { files: DocFiles::default() }
    }
    // fn print_strings(&mut self) {
    //     for i in self.files.clone() {
    //         println!("{}", i)
    //     }
    // }
    // fn add_file<T: ToString>(&mut self, string: T) {
    //     self.files
    // }
    // fn remote_string(&mut self, string: String) {
    //     let mut pos = 0;
    //     for i in self.files.clone() {
    //         if i.contains(&string) {
    //             self.files.remove(pos);
    //         }
    //         pos += 1
    //     }
    // }
}




fn main() {
    let state = Mutex::new(AppState::default());



    tauri::Builder::default()
        // .plugin(tauri_plugin_dialog::init())
        .setup( |app| {
          app.manage(state);
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            // window.close_devtools();
            Ok(())
        })
        .invoke_handler(
          tauri::generate_handler![date, printall, get_entries, remove_entry, remove_all_entries, open_file, update_word_count]
        )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
