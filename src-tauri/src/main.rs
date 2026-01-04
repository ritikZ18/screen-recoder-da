// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capture;
mod encoder;
mod session;
mod analytics;
mod observability;
mod system_metrics;

use session::SessionManager;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    session_manager: Arc<Mutex<SessionManager>>,
}

#[tauri::command]
async fn list_monitors(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    tracing::info!("list_monitors command called");
    let manager = state.session_manager.lock().await;
    let result = manager.list_monitors().await;
    tracing::info!("list_monitors result: {:?}", result);
    result
}

#[tauri::command]
async fn list_windows(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    tracing::info!("list_windows command called");
    let manager = state.session_manager.lock().await;
    let result = manager.list_windows().await;
    tracing::info!("list_windows result: {:?}", result);
    result
}

#[tauri::command]
async fn start_recording(
    monitor_id: Option<String>,
    window_id: Option<String>,
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut manager = state.session_manager.lock().await;
    manager
        .start_recording(monitor_id, window_id, app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn stop_recording(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut manager = state.session_manager.lock().await;
    manager.stop_recording(Some(app)).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn pause_recording(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut manager = state.session_manager.lock().await;
    manager.pause_recording().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_recording_status(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let manager = state.session_manager.lock().await;
    manager.get_recording_status().await
}

#[tauri::command]
async fn get_timeline_data(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let manager = state.session_manager.lock().await;
    manager.get_timeline_data().await
}

#[tokio::main]
async fn main() {
    // Initialize observability
    observability::init().expect("Failed to initialize observability");

    // Initialize session manager
    let session_manager = Arc::new(Mutex::new(SessionManager::new().await));

    let app_state = AppState { session_manager };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_monitors,
            list_windows,
            start_recording,
            stop_recording,
            pause_recording,
            get_recording_status,
            get_timeline_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

