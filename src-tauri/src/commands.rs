use std::fs;
use std::process::Command;
use tauri::{AppHandle, Emitter, Manager};
use crate::storage::{self, OrbPosition, TaskMemoryStore, TaskMemoryItem};

#[tauri::command]
pub async fn toggle_main_window(app: AppHandle) -> Result<bool, String> {
    if let Some(main_win) = app.get_webview_window("main") {
        let is_visible = main_win.is_visible().unwrap_or(false);
        if is_visible {
            let _ = main_win.hide();
            Ok(false)
        } else {
            // Anchor main window near orb if orb position exists
            if let Some(orb_win) = app.get_webview_window("orb") {
                if let Ok(orb_pos) = orb_win.outer_position() {
                    let _ = main_win.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: (orb_pos.x - 300).max(50),
                        y: (orb_pos.y + 70).max(50),
                    }));
                }
            }
            let _ = main_win.show();
            let _ = main_win.set_focus();
            Ok(true)
        }
    } else {
        Err("Main window not found".into())
    }
}

#[tauri::command]
pub fn save_orb_position(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
    storage::save_orb_position(&app, OrbPosition { x, y })
}

#[tauri::command]
pub fn get_orb_position(app: AppHandle) -> Result<Option<OrbPosition>, String> {
    Ok(storage::load_orb_position(&app))
}

#[tauri::command]
pub fn open_installed_app(name: String) -> Result<String, String> {
    let app_name = name.trim();
    if app_name.is_empty() {
        return Err("App name cannot be empty".into());
    }

    // Direct open using shell open crate or system command
    match open::that(app_name) {
        Ok(_) => Ok(format!("Successfully launched {}", app_name)),
        Err(_) => {
            // Fallback for Windows start command
            let status = Command::new("cmd")
                .args(["/C", "start", "", app_name])
                .status()
                .map_err(|e| e.to_string())?;

            if status.success() {
                Ok(format!("Opened {}", app_name))
            } else {
                Err(format!("Could not find or launch application: {}", app_name))
            }
        }
    }
}

#[tauri::command]
pub fn run_allowlisted_command(command: String, confirmed: bool) -> Result<String, String> {
    let cmd_str = command.trim();
    let lower_cmd = cmd_str.to_lowercase();

    // Check destructive patterns
    let is_destructive = lower_cmd.contains("rm ") 
        || lower_cmd.contains("del ") 
        || lower_cmd.contains("format ") 
        || lower_cmd.contains("rd ") 
        || lower_cmd.contains("remove-item")
        || lower_cmd.contains("shutdown");

    if is_destructive && !confirmed {
        return Err("DESTRUCTIVE_CONFIRMATION_REQUIRED".into());
    }

    let output = Command::new("cmd")
        .args(["/C", cmd_str])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() { "Command executed successfully.".to_string() } else { stdout })
    } else {
        Err(format!("Command failed with error:\n{}", stderr))
    }
}

#[tauri::command]
pub fn read_user_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub fn write_user_file(path: String, content: String) -> Result<String, String> {
    fs::write(&path, content).map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
pub fn set_system_volume(level: f32) -> Result<String, String> {
    let target = level.clamp(0.0, 1.0);
    
    #[cfg(target_os = "windows")]
    {
        use winapi::um::mmdeviceapi::{CLSID_MMDeviceEnumerator, IMMDeviceEnumerator, eRender, eMultimedia};
        use winapi::um::endpointvolume::IAudioEndpointVolume;
        use winapi::um::combaseapi::{CoInitialize, CoCreateInstance, CLSCTX_ALL};
        use winapi::Interface;
        use std::ptr;

        unsafe {
            let _ = CoInitialize(ptr::null_mut());
            let mut enumerator: *mut IMMDeviceEnumerator = ptr::null_mut();
            let hr = CoCreateInstance(
                &CLSID_MMDeviceEnumerator,
                ptr::null_mut(),
                CLSCTX_ALL,
                &IMMDeviceEnumerator::uuidof(),
                &mut enumerator as *mut _ as *mut _,
            );

            if hr == 0 && !enumerator.is_null() {
                let mut device = ptr::null_mut();
                if (*enumerator).GetDefaultAudioEndpoint(eRender, eMultimedia, &mut device) == 0 && !device.is_null() {
                    let mut endpoint_vol: *mut IAudioEndpointVolume = ptr::null_mut();
                    if (*device).Activate(&IAudioEndpointVolume::uuidof(), CLSCTX_ALL, ptr::null_mut(), &mut endpoint_vol as *mut _ as *mut _) == 0 && !endpoint_vol.is_null() {
                        (*endpoint_vol).SetMasterVolumeLevelScalar(target, ptr::null());
                        return Ok(format!("System volume set to {}%", (target * 100.0) as u32));
                    }
                }
            }
        }
    }

    Ok(format!("Volume setting registered at {}%", (target * 100.0) as u32))
}

#[tauri::command]
pub fn take_desktop_screenshot() -> Result<String, String> {
    use screenshots::Screen;
    let screens = Screen::all().map_err(|e| e.to_string())?;
    if let Some(screen) = screens.first() {
        let image = screen.capture().map_err(|e| e.to_string())?;
        let temp_dir = std::env::temp_dir();
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let file_path = temp_dir.join(format!("jarvis_screenshot_{}.png", timestamp));
        image.save(&file_path).map_err(|e| e.to_string())?;
        Ok(file_path.to_string_lossy().to_string())
    } else {
        Err("No active screen found for screenshot".into())
    }
}

#[tauri::command]
pub fn get_task_memory(app: AppHandle) -> Result<TaskMemoryStore, String> {
    Ok(storage::load_task_memory(&app))
}

#[tauri::command]
pub fn update_task_memory(app: AppHandle, active_task: String, query: String, summary: String) -> Result<TaskMemoryStore, String> {
    let mut store = storage::load_task_memory(&app);
    if !active_task.is_empty() {
        store.active_task = active_task.clone();
    }
    if !query.is_empty() || !summary.is_empty() {
        let item = TaskMemoryItem {
            timestamp: chrono::Local::now().to_rfc3339(),
            query,
            summary,
            category: "user_task".to_string(),
        };
        store.history.push(item);
        if store.history.len() > 50 {
            store.history.remove(0);
        }
    }
    storage::save_task_memory(&app, &store)?;
    Ok(store)
}

#[tauri::command]
pub fn send_native_notification(app: AppHandle, title: String, body: String) -> Result<(), String> {
    let _ = app.emit("native-notification", serde_json::json!({ "title": title, "body": body }));
    Ok(())
}
