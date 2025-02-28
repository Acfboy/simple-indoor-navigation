use std::sync::MutexGuard;

use navigator::guidance::{Route, ScreenSize};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_android_fs::{AndroidFs, AndroidFsExt};
use tauri_plugin_fs::FsExt;
use tauri_plugin_mobilesensors;

mod navigator;
use navigator::map::{Map, Node};
use navigator::{guidance::Guidance, map::Position};
// use navigator::map::{Map, Mark};

#[derive(Default)]
struct State {
    map: std::sync::Mutex<Map>,
    guidance: std::sync::Mutex<Guidance>,
    imgs: std::sync::Mutex<Vec<String>>,
    scales: std::sync::Mutex<Vec<f64>>,
}

#[tauri::command]
fn clear_data(state: tauri::State<'_, State>) -> Result<(), String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    *data = Map::default();
    let mut data = state.guidance.lock().map_err(|e| e.to_string())?;
    *data = Guidance::default();
    Ok(())
}

#[tauri::command]
fn add_edge(state: tauri::State<'_, State>, from: usize, to: usize) -> Result<(), String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    (*data).add_edge(from, to)
}

#[tauri::command]
fn add_node(state: tauri::State<'_, State>, pos: Position, floor: usize) -> Result<usize, String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    Ok((*data).add_node(pos, floor))
}

#[tauri::command]
fn remove_node(state: tauri::State<'_, State>, index: usize) -> Result<(), String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    (*data).remove_node(index)
}

#[tauri::command]
fn remove_edge(state: tauri::State<'_, State>, from: usize, to: usize) -> Result<(), String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    (*data).remove_edge(from, to)
}

#[tauri::command]
fn mark_node(
    state: tauri::State<'_, State>,
    index: usize,
    name: String,
    elevator: String,
) -> Result<(), String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    (*data).mark_node(index, name, elevator)
}

#[tauri::command]
fn cur_node_list(state: tauri::State<'_, State>) -> Result<Vec<Node>, String> {
    let data = state.map.lock().map_err(|e| e.to_string())?;
    Ok(data.nodes.clone())
}

use tauri_plugin_android_fs::VisualMediaTarget;

#[tauri::command]
async fn select_image(app: AppHandle) -> Result<(Vec<u8>, String), String> {
    let api = app.android_fs();
    let select = api
        .show_open_visual_media_dialog(VisualMediaTarget::ImageOnly, false)
        .map_err(|e| e.to_string())?;
    if !select.is_empty() {
        let file_path = &select[0];
        let content = api.read(file_path).map_err(|e| e.to_string())?;
        Ok((content, api.get_mime_type(file_path).unwrap().unwrap()))
    } else {
        Err(String::new())
    }
}

#[derive(Serialize, Deserialize)]
struct StoreData {
    imgs: Vec<String>,
    scales: Vec<f64>,
    map: Map,
}

#[tauri::command]
async fn get_store_data(
    imgs: Vec<String>,
    scales: Vec<f64>,
    state: tauri::State<'_, State>,
) -> Result<String, String> {
    let map = state.map.lock().map_err(|e| e.to_string())?;
    let data = StoreData {
        imgs,
        scales,
        map: map.clone(),
    };
    Ok(serde_json::to_string(&data).unwrap())
}

fn update_nav(
    app: &AppHandle,
    route: Route,
    images: MutexGuard<Vec<String>>,
    scale: MutexGuard<Vec<f64>>,
) -> Result<(), String> {
    app.emit("update-route", route.clone())
        .map_err(|e| e.to_string())?;
    app.emit("update-image", (*images)[route.0[0].floor - 1].clone())
        .map_err(|e| e.to_string())?;
    if route.0.len() == 2 && route.0[0].floor != route.0[1].floor {
        app.emit("update-scale", (*scale)[route.0[1].floor - 1])
            .map_err(|e| e.to_string())?;
        app.emit(
            "prompt",
            format!(
                "由 {} 上 {} 楼。",
                route.0[0].name,
                route.0[route.0.len() - 1].floor
            ),
        )
        .map_err(|e| e.to_string())?;
    }
    app.emit(
        "prompt",
        format!(
            "按图示从 {} 走到 {}。",
            route.0[0].name,
            route.0[route.0.len() - 1].name
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}


/// 从 `from` 走到最近的名称为 `to` 的点。
#[tauri::command]
fn create_new_nav(
    app: AppHandle,
    state: tauri::State<'_, State>,
    from: usize,
    to: String,
    map: Map,
    imgs: Vec<String>,
    scale: Vec<f64>,
    screen: ScreenSize,
    disable_elevator: bool
) -> Result<(), String> {
    let mut data = state.map.lock().map_err(|e| e.to_string())?;
    (*data) = map;
    let guide = (*data).navigate(from, to, disable_elevator)?;
    let mut images = state.imgs.lock().map_err(|e| e.to_string())?;
    (*images) = imgs;
    let mut scales = state.scales.lock().map_err(|e| e.to_string())?;
    (*scales) = scale;
    let mut data = state.guidance.lock().map_err(|e| e.to_string())?;
    (*data) = guide;
    (*data).step_by_step(screen, (*scales).clone())?;
    let fisrt_route = (*data).query()?;
    app.emit("update-image", fisrt_route.0[0].clone())
        .map_err(|e| e.to_string())?;
    update_nav(&app, fisrt_route, images, scales)?;
    app.emit("begin", ()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn next_step(app: AppHandle, state: tauri::State<'_, State>) -> Result<(), String> {
    let images = state.imgs.lock().map_err(|e| e.to_string())?;
    let scales = state.scales.lock().map_err(|e| e.to_string())?;
    let mut data = state.guidance.lock().map_err(|e| e.to_string())?;
    (*data).next_step()?;
    update_nav(&app, (*data).query()?, images, scales)?;
    Ok(())
}

#[tauri::command]
fn prev_step(app: AppHandle, state: tauri::State<'_, State>) -> Result<(), String> {
    let images = state.imgs.lock().map_err(|e| e.to_string())?;
    let scales = state.scales.lock().map_err(|e| e.to_string())?;
    let mut data = state.guidance.lock().map_err(|e| e.to_string())?;
    (*data).prev_step()?;
    update_nav(&app, (*data).query()?, images, scales)?;
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
fn import_map(app: AppHandle) -> Result<String, ()> {
    let api = app.android_fs();
    let select = api.show_open_file_dialog(&["*/*"], false).unwrap();
    if !select.is_empty() {
        let file_path = &select[0];
        let content = api.read(file_path).unwrap();
        if let Ok(s) = String::from_utf8(content) {
            let parsed = serde_json::from_str::<StoreData>(&s);
            if let Ok(data) = parsed {
                if data.map.is_valid() {
                    return Ok(s);
                } else {
                    return Err(());
                }
            } else {
                return Err(());
            }
        } else {
            return Err(());
        }
    }
    Ok(String::new())
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
            clear_data,
            add_edge,
            add_node,
            remove_node,
            remove_edge,
            mark_node,
            cur_node_list,
            export_map,
            import_map,
            select_image,
            get_store_data,
            create_new_nav,
            prev_step,
            next_step
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
