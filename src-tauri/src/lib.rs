use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_fs::FsExt;
use tauri_plugin_mobilesensors;

mod navigator;
use navigator::map::{Map, Mark};
use navigator::Navigator;

#[derive(Default)]
struct State {
    s: std::sync::Mutex<Navigator>,
}

#[derive(Serialize, Clone)]
struct TimelineResponse {
    start: String,
    dest: String,
    path: Vec<String>
}

#[tauri::command]
async fn create_new_nav(
    app: AppHandle,
    state: tauri::State<'_, State>,
    cur: Mark,
    dest: Mark,
    map: Map,
) -> Result<(), ()> {
    let mut data = state.s.lock().unwrap();
    (*data).init(map);
    (*data).navigate(&cur, &dest)?;
    let timeline = data.route().timeline();
    app.emit("route-update", TimelineResponse {
        start: cur.full_name(),
        dest: dest.full_name(),
        path: timeline
    }).unwrap();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(State::default())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_mobilesensors::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let dir = app.path().data_dir().unwrap().join("maps");
            println!("Config Directory: {:?}", dir);
            scope.allow_directory(dir, false).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_new_nav])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
