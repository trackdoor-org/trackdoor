use std::sync::Mutex;
use tauri::{Manager};
use commands::AppState as AppState;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_new_file,
            commands::close_gpx_file,
            commands::open_gpx_file,
            commands::get_gpx_files,
            commands::save_gpx_file,
            commands::get_selected_file_idx,
            commands::select_gpx_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
