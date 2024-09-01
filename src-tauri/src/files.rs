use std::sync::Mutex;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use tauri::State;
use crate::AppState;

#[tauri::command]
pub fn open_file(state: State<'_, Mutex<AppState>>) {
    // let mut app_state = state.lock().unwrap();
    // app_state.strings.push(string);
    file_dialog();
}

pub fn file_dialog() {
    let path = FileDialog::new()
        // .set_location("~/Desktop")
        // .add_filter("PNG Image", &["png"])
        // .add_filter("JPEG Image", &["jpg", "jpeg"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        // do_something(path);
        println!("file: {:?}", path.as_path())
    }
}
