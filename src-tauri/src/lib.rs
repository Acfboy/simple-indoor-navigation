use tauri_plugin_mobilesensors;
use tauri_plugin_fs::FsExt;
use tauri::Manager;
// use tauri_plugin_fs::FsExt;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_mobilesensors::init())
        .setup(|app| {
            // 获取当前应用程序的文件系统权限范围
            let scope = app.fs_scope();
            
            // 获取 BaseDirectory.AppConfig 目录的路径  
            let config_dir = app.path().data_dir().unwrap().join("maps");
            // let config_dir = PathResolver().config_dir().unwrap();
            println!("Config Directory: {:?}", config_dir);
            
            // 允许访问 BaseDirectory.AppConfig 目录
            scope.allow_directory(config_dir, false).unwrap();
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
