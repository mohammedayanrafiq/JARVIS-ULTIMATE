// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::command;
use std::fs;
use sysinfo::{System, SystemExt, ProcessExt};
use screenshots::Screen;
use base64::Engine;
use base64::engine::general_purpose;
use enigo::{Enigo, MouseControllable, KeyboardControllable, MouseButton};

#[command]
fn execute_shell(command: String) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-Command", &command])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
    };

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            if out.status.success() {
                Ok(stdout)
            } else {
                Err(format!("Error: {}", stderr))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
fn read_file_content(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[command]
fn write_file_content(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[command]
fn list_applications() -> Result<Vec<String>, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut procs = Vec::new();
    for (_pid, process) in sys.processes() {
        procs.push(process.name().to_string());
    }
    procs.sort();
    procs.dedup();
    Ok(procs)
}

#[command]
fn kill_application(name: String) -> Result<(), String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut killed = false;
    for (_pid, process) in sys.processes() {
        if process.name().to_lowercase().contains(&name.to_lowercase()) {
            process.kill();
            killed = true;
        }
    }
    if killed {
        Ok(())
    } else {
        Err(format!("Process containing '{}' not found", name))
    }
}

#[command]
fn capture_screenshot() -> Result<String, String> {
    let screens = Screen::all().map_err(|e| e.to_string())?;
    if let Some(screen) = screens.first() {
        let image = screen.capture().map_err(|e| e.to_string())?;
        let bytes = image.to_png().map_err(|e| e.to_string())?;
        let b64 = general_purpose::STANDARD.encode(&bytes);
        Ok(format!("data:image/png;base64,{}", b64))
    } else {
        Err("No screens found".to_string())
    }
}

#[command]
fn enigo_mouse_move(x: i32, y: i32) -> Result<(), String> {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    Ok(())
}

#[command]
fn enigo_mouse_click(button: String) -> Result<(), String> {
    let mut enigo = Enigo::new();
    let btn = match button.to_lowercase().as_str() {
        "left" => MouseButton::Left,
        "right" => MouseButton::Right,
        "middle" => MouseButton::Middle,
        _ => MouseButton::Left,
    };
    enigo.mouse_click(btn);
    Ok(())
}

#[command]
fn enigo_type(text: String) -> Result<(), String> {
    let mut enigo = Enigo::new();
    enigo.key_sequence(&text);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            execute_shell,
            read_file_content,
            write_file_content,
            list_applications,
            kill_application,
            capture_screenshot,
            enigo_mouse_move,
            enigo_mouse_click,
            enigo_type
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
