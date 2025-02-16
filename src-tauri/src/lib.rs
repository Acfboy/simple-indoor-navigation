use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_android_fs::{AndroidFs, AndroidFsExt};
use tauri_plugin_fs::FsExt;
use tauri_plugin_mobilesensors;

mod navigator;
use navigator::guidance::Guidance;
use navigator::map::{Map, Mark};
use navigator::Navigator;

mod response;
use response::{IntersectionResponse, TimelineResponse};

#[derive(Default)]
struct State {
    s: std::sync::Mutex<Navigator>,
}

fn update_nav(app: &AppHandle, nav: Guidance) {
    app.emit(
        "landmark-change",
        IntersectionResponse {
            landmarks: nav.next_marks,
        },
    )
    .unwrap();
    app.emit("prompt", nav.prompt).unwrap();
    app.emit("target-change", nav.target_direction).unwrap();
}

#[tauri::command]
async fn create_new_nav(
    app: AppHandle,
    state: tauri::State<'_, State>,
    cur: Mark,
    dest: Mark,
    map: Map,
) -> Result<(), String> {
    let mut data = state.s.lock().map_err(|e| e.to_string())?;
    (*data).init(map.clone());
    (*data).navigate(&cur, &dest)?;
    let timeline = data.route().timeline();
    app.emit(
        "route-change",
        TimelineResponse {
            start: cur.full_name(),
            dest: dest.full_name(),
            path: timeline,
        },
    )
    .unwrap();
    update_nav(&app, data.route().query());
    Ok(())
}

#[tauri::command]
async fn next_step(app: AppHandle, state: tauri::State<'_, State>) -> Result<(), ()> {
    let mut data = state.s.lock().unwrap();
    data.route().next_guidance()?;
    update_nav(&app, data.route().query());
    app.emit("route-move", 1).unwrap();
    Ok(())
}

#[tauri::command]
async fn prev_step(app: AppHandle, state: tauri::State<'_, State>) -> Result<(), ()> {
    let mut data = state.s.lock().unwrap();
    data.route().prev_guidance()?;
    update_nav(&app, data.route().query());
    app.emit("route-move", -1).unwrap();
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
        .invoke_handler(tauri::generate_handler![
            create_new_nav,
            export_map,
            import_map,
            next_step,
            prev_step
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
