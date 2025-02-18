use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_android_fs::{AndroidFs, AndroidFsExt};
use tauri_plugin_fs::FsExt;
use tauri_plugin_mobilesensors;

mod navigator;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_android_fs::init())
        .plugin(tauri_plugin_mobilesensors::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let data_dir = app.path().data_dir().unwrap().join("maps");
            scope.allow_directory(data_dir, false).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
