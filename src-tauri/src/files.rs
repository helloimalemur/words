use std::sync::Mutex;
use native_dialog::{FileDialog, MessageDialog, MessageType};
use os_info::Type;
use rand::Rng;
use tauri::State;
use crate::AppState;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DocFiles {
    pub files_container: Vec<DocFile>
}

impl DocFiles {
    pub fn default() -> DocFiles {
        DocFiles { files_container: vec![] } }
    pub fn remove(&mut self, ind: u32) {
        if !self.files_container.is_empty() {
            let mut tr = vec![];
            for (i, d) in self.files_container.iter().enumerate() {
                if d.index.eq(&ind) {
                    tr.push(i)
                }
            }
            for ent in tr {
                self.files_container.remove(ent);
            }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DocFile {
    pub index: u32,
    pub name: String,
    pub title: String,
    pub path: String,
    pub word_count: u64
}

impl DocFile {
    pub fn new(path: String) -> DocFile {
        let path_string = String::from(path);
        let mut rand = rand::thread_rng();
        let ind: u32 = rand.random();

        DocFile {
            index: ind,
            path: path_string,
            name: "".to_string(),
            word_count: 0,
            title: "".to_string(),
        }
    }
}

#[tauri::command]
pub fn open_file(state: State<'_, Mutex<AppState>>) {
    // let mut app_state = state.lock().unwrap();
    // app_state.strings.push(string);
    file_dialog(state);
}

pub fn file_dialog(state: State<Mutex<AppState>>) {
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
        let path_string = path.as_path().to_string_lossy().to_string();
        let file_name = file_name_from_full_path(path_string.clone()).unwrap_or_default();
        println!("loading file: {:?} {:?}", path_string, file_name);
        add_file(path_string, state);
    }
}

pub fn add_file(path: String, state: State<Mutex<AppState>>) {
    let mut app_state = state.lock().unwrap();
    app_state.files.files_container.push(
        DocFile::new(path)
    )
}

fn file_name_from_full_path(path: String) -> Option<String> {
    // get file name only
    let info = os_info::get();
    let path: String = match info.os_type() {
        Type::Windows => path.split('\\').last()?,
        _ => path.split('/').last()?,
    }
    .to_string();
    Some(path)
}
