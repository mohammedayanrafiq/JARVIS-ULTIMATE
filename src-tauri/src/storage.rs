use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrbPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TaskMemoryItem {
    pub timestamp: String,
    pub query: String,
    pub summary: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TaskMemoryStore {
    pub active_task: String,
    pub history: Vec<TaskMemoryItem>,
}

fn get_data_dir(app: &AppHandle) -> PathBuf {
    use tauri::Manager;
    let path = app
        .path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from(".jarvis_data"));
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn load_orb_position(app: &AppHandle) -> Option<OrbPosition> {
    let mut file_path = get_data_dir(app);
    file_path.push("orb_position.json");
    if let Ok(content) = fs::read_to_string(file_path) {
        serde_json::from_str(&content).ok()
    } else {
        None
    }
}

pub fn save_orb_position(app: &AppHandle, pos: OrbPosition) -> Result<(), String> {
    let mut file_path = get_data_dir(app);
    file_path.push("orb_position.json");
    let json_data = serde_json::to_string_pretty(&pos).map_err(|e| e.to_string())?;
    fs::write(file_path, json_data).map_err(|e| e.to_string())
}

pub fn load_task_memory(app: &AppHandle) -> TaskMemoryStore {
    let mut file_path = get_data_dir(app);
    file_path.push("task_memory.json");
    if let Ok(content) = fs::read_to_string(file_path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        TaskMemoryStore::default()
    }
}

pub fn save_task_memory(app: &AppHandle, store: &TaskMemoryStore) -> Result<(), String> {
    let mut file_path = get_data_dir(app);
    file_path.push("task_memory.json");
    let json_data = serde_json::to_string_pretty(store).map_err(|e| e.to_string())?;
    fs::write(file_path, json_data).map_err(|e| e.to_string())
}
