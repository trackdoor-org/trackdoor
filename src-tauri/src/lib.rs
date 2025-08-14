use std::sync::Mutex;
use std::path::PathBuf;
use std::time::Duration;
use std::thread::{sleep, spawn};
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use serde::{Serialize, Deserialize};
use tauri::{ipc::Channel, AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use gpx::{Gpx, GpxVersion};


#[derive(Default)]
struct AppState {
    gpx_files: Vec<GpxFile>
}

#[derive(Clone, Serialize, Deserialize)]
struct GpxFile {
    path: String,
    name: String,
    is_saved: bool,
    gpx: Gpx,
}


#[tauri::command]
fn create_new_file(app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.gpx_files.push(
        GpxFile {
            path: "".to_string(),
            name: "Untitled".to_string(),
            is_saved: false,
            gpx: Gpx {
                version: GpxVersion::Gpx11,
                creator: None,
                metadata: None,
                waypoints: vec![],
                tracks: vec![],
                routes: vec![],
            }
        }
    );
}


#[tauri::command]
async fn close_gpx_file(index: usize, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut should_save: bool= false;

    {
        let mut state = state.lock().unwrap();

        if !state.gpx_files[index].is_saved {
            should_save = app.dialog()
                .message("Do you want to save the changes?")
                .title("Unsaved changes")
                .buttons(MessageDialogButtons::YesNo)
                .blocking_show();

            if !should_save {
                state.gpx_files.remove(index);
                return;
            }
        }
    }

    if should_save {
        save_gpx_file(index, app.clone()).await;

        let mut state = state.lock().unwrap();
        if state.gpx_files[index].is_saved {
            state.gpx_files.remove(index);
        }
    } else {
        let mut state = state.lock().unwrap();
        state.gpx_files.remove(index);
    }
}


#[tauri::command]
async fn open_gpx_file(path_str: String, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    let file = File::open(&path_str);

    let file: File = match file {
        Ok(file) => file,
        Err(error) => {
            show_error_popup(&app, &error.to_string());
            return;
        }
    };
    
    let reader = BufReader::new(file);
    let gpx = gpx::read(reader);

    let gpx: Gpx = match gpx {
        Ok(gpx) => gpx,
        Err(error) => {
            show_error_popup(&app, &error.to_string());
            return;
        },
    };

    let path = Path::new(&path_str);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    state.gpx_files.push(
        GpxFile {
            path: path_str.to_string(),
            name: file_name.to_string(),
            is_saved: true,
            gpx: gpx,
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


#[tauri::command]
async fn save_gpx_file(index: usize, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    let gpx_file = &mut state.gpx_files[index];

    if gpx_file.path == "" {
        let file_path = app
            .dialog()
            .file()
            .add_filter("gpx", &["gpx"])
            .add_filter("All files", &["*"])
            .blocking_save_file();
        
        let file_path = match file_path {
            Some(file_path) => file_path.into_path(),
            None => {
                return;
            },
        };

        let mut file_path: PathBuf = match file_path {
            Ok(file_path) => file_path,
            Err(error) => {
                show_error_popup(&app, &error.to_string());
                return;
            },
        };

        file_path.set_extension("gpx");

        gpx_file.path = file_path.to_str().unwrap().to_string();
        gpx_file.name = file_path.file_name().expect("REASON").to_str().unwrap().to_string();
    }

    println!("{}", gpx_file.path);
    // TODO: Write to file
    
    gpx_file.is_saved = true;
}


fn show_error_popup(app: &AppHandle, error_msg: &str) {
    app.dialog()
    .message(format!("Error: {}", error_msg))
    .kind(MessageDialogKind::Error)
    .title("Error")
    .blocking_show();
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
        .invoke_handler(tauri::generate_handler![create_new_file, close_gpx_file, open_gpx_file, get_gpx_files, save_gpx_file,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
