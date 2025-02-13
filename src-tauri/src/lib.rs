use tauri_plugin_mobilesensors;
use tauri_plugin_fs::FsExt;
use tauri::Manager;

mod navigator;
use navigator::Navigator;
// use tauri_plugin_fs::FsExt;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_mobilesensors::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let dir = app.path().data_dir().unwrap().join("maps");
            println!("Config Directory: {:?}", dir);
            scope.allow_directory(dir, false).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
