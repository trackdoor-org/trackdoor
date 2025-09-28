use geojson::{GeoJson, Value, Feature, FeatureCollection, Position};
use gpx::{Gpx, GpxVersion};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;
use tauri::{ipc::Channel, AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

#[derive(Default)]
pub struct AppState {
    gpx_files: Vec<GpxFile>,
    selected_file_idx: usize,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpxFile {
    path: String,
    name: String,
    is_saved: bool,
    geo_json_str: String,
    gpx: Gpx,
}

#[tauri::command]
pub fn create_new_file(app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.gpx_files.push(GpxFile {
        path: "".to_string(),
        name: "Untitled".to_string(),
        is_saved: false,
        geo_json_str: "".to_string(),
        gpx: Gpx {
            version: GpxVersion::Gpx11,
            creator: None,
            metadata: None,
            waypoints: vec![],
            tracks: vec![],
            routes: vec![],
        },
    });
}

#[tauri::command]
pub async fn close_gpx_file(index: usize, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut should_save: bool = false;

    {
        let mut state = state.lock().unwrap();

        if !state.gpx_files[index].is_saved {
            should_save = app
                .dialog()
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
pub async fn open_gpx_file(path_str: String, app: AppHandle) {
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
        }
    };

    let path = Path::new(&path_str);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    state.gpx_files.push(GpxFile {
        path: path_str.to_string(),
        name: file_name.to_string(),
        is_saved: true,
        geo_json_str: convert_gpx_to_geo_json(&gpx).to_string(),
        gpx: gpx,
    });
}

#[tauri::command]
pub fn get_gpx_files(app: AppHandle, on_gpx_data_recived: Channel<Vec<GpxFile>>) {
    spawn(move || loop {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        on_gpx_data_recived.send(state.gpx_files.clone()).unwrap();

        drop(state);

        sleep(Duration::from_millis(100))
    });
}

#[tauri::command]
pub fn get_selected_file_idx(app: AppHandle, on_selected_file_idx_recived: Channel<usize>) {
    spawn(move || loop {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        on_selected_file_idx_recived.send(state.selected_file_idx.clone()).unwrap();

        drop(state);

        sleep(Duration::from_millis(100))
    });
}

#[tauri::command]
pub async fn select_gpx_file(index: usize, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.selected_file_idx = index;
}

#[tauri::command]
pub async fn save_gpx_file(index: usize, app: AppHandle) {
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
            }
        };

        let mut file_path: PathBuf = match file_path {
            Ok(file_path) => file_path,
            Err(error) => {
                show_error_popup(&app, &error.to_string());
                return;
            }
        };

        file_path.set_extension("gpx");

        gpx_file.path = file_path.to_str().unwrap().to_string();
        gpx_file.name = file_path
            .file_name()
            .expect("REASON")
            .to_str()
            .unwrap()
            .to_string();
    }

    let file = File::create(&gpx_file.path);
    let file: File = match file {
        Ok(file) => file,
        Err(error) => {
            show_error_popup(&app, &error.to_string());
            return;
        }
    };

    let _ = gpx::write(&gpx_file.gpx, file);

    gpx_file.is_saved = true;
}

fn convert_gpx_to_geo_json(gpx: &Gpx) -> GeoJson {
    let mut coords = Vec::new();
    for track in &gpx.tracks {
        for segment in &track.segments {
            for point in &segment.points {
                coords.push(Position::from(vec![point.point().x(), point.point().y()]));
            }
        }
    }

    let line_string = Value::LineString(coords);
    let feature = Feature {
        geometry: Some(line_string.into()),
        ..Default::default()
    };

    let geojson = GeoJson::FeatureCollection(FeatureCollection {
        features: vec![feature],
        ..Default::default()
    });

    return geojson;
}

fn show_error_popup(app: &AppHandle, error_msg: &str) {
    app.dialog()
        .message(format!("Error: {}", error_msg))
        .kind(MessageDialogKind::Error)
        .title("Error")
        .blocking_show();
}
