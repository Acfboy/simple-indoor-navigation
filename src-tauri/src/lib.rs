use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_fs::FsExt;
use tauri_plugin_mobilesensors;
use tauri_plugin_android_fs::{AndroidFs, AndroidFsExt};

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
    path: Vec<String>,
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
    app.emit(
        "route-update",
        TimelineResponse {
            start: cur.full_name(),
            dest: dest.full_name(),
            path: timeline,
        },
    )
    .unwrap();
    Ok(())
}

#[tauri::command]
fn export_map(app: AppHandle, content: String, name: String) {
    let api = app.android_fs();
    let select = api.show_save_file_dialog(name, None).unwrap();
    if let Some(file_path) = select {
        api.create_file(&file_path).unwrap();
        api.write(&file_path, content).unwrap();
    }
}

#[tauri::command]
fn import_map(app: AppHandle) -> String {
    let api = app.android_fs();
    let select = api.show_open_file_dialog(&["*/*"], false).unwrap();
    if !select.is_empty() {
        let file_path = &select[0];
        let content = api.read(file_path).unwrap();
        if let Ok(s) = String::from_utf8(content) {
            let parsed = serde_json::from_str::<Map>(&s);
            if let Ok(_) = parsed {
                return s;
            } else {
                return String::from("err");
            }
        } else {
            return String::from("err");
        }
    }
    String::new()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(State::default())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_android_fs::init())
        .plugin(tauri_plugin_mobilesensors::init())
        .setup(|app| {
            let scope = app.fs_scope();
            let data_dir = app.path().data_dir().unwrap().join("maps");
            scope.allow_directory(data_dir, false).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_new_nav, export_map, import_map])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
