use std::sync::Mutex;
use std::time::Duration;
use std::thread::{sleep, spawn};
use std::path::Path;
use serde::{Serialize, Deserialize};
use tauri::{ipc::Channel, AppHandle, Manager};


#[derive(Default)]
struct AppState {
    gpx_files: Vec<GpxFile>
}

#[derive(Clone, Serialize, Deserialize)]
struct GpxFile {
    path: String,
    name: String,
}


#[tauri::command]
fn create_new_file(app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.gpx_files.push(
        GpxFile {
            path: "path".to_string(),
            name: "new-file".to_string(),
        }
    );

    let serialized_data = serde_json::to_string(&state.gpx_files).unwrap();
    println!("{}", serialized_data);
}


#[tauri::command]
fn open_gpx_file(path_str: String, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    let path = Path::new(&path_str);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    state.gpx_files.push(
        GpxFile {
            path: path_str.to_string(),
            name: file_name.to_string(),
        }
    );
}


#[tauri::command]
fn get_gpx_files(app: AppHandle, on_event: Channel<Vec<GpxFile>>) {
    spawn(move || {
        loop {
            let state = app.state::<Mutex<AppState>>();
            let state = state.lock().unwrap();

            on_event.send(state.gpx_files.clone()).unwrap();

            drop(state);

            sleep(Duration::from_millis(100))
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
        .invoke_handler(tauri::generate_handler![create_new_file, open_gpx_file, get_gpx_files,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
