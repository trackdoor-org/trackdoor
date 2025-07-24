use std::sync::Mutex;
use std::time::Duration;
use std::thread::{sleep, spawn};
use serde::{Serialize, Deserialize};
use tauri::{ipc::Channel, AppHandle, Manager};


#[derive(Default)]
struct AppState {
    files: Vec<File>
}

#[derive(Clone, Serialize, Deserialize)]
struct File {
    path: String,
    name: String,
}


#[tauri::command]
fn create_new_file(app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.files.push(
        File {
            path: "path".to_string(),
            name: "new-file".to_string(),
        }
    );

    let serialized_data = serde_json::to_string(&state.files).unwrap();
    println!("{}", serialized_data);
}


#[tauri::command]
fn open_file(path: &str, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.files.push(
        File {
            path: path.to_string(),
            name: "new-file".to_string(),
        }
    );
}


#[tauri::command]
fn get_files(app: AppHandle, on_event: Channel<Vec<File>>) {
    spawn(move || {
        loop {
            let state = app.state::<Mutex<AppState>>();
            let state = state.lock().unwrap();

            on_event.send(state.files.clone()).unwrap();

            sleep(Duration::from_secs(1))
        }
    });
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_new_file, open_file, get_files,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
