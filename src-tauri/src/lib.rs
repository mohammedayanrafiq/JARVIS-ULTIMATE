pub mod commands;
pub mod storage;
pub mod voice;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use voice::VoiceListener;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::AppleScript,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::toggle_main_window,
            commands::save_orb_position,
            commands::get_orb_position,
            commands::open_installed_app,
            commands::run_allowlisted_command,
            commands::read_user_file,
            commands::write_user_file,
            commands::set_system_volume,
            commands::take_desktop_screenshot,
            commands::get_task_memory,
            commands::update_task_memory,
            commands::send_native_notification
        ])
        .setup(|app| {
            // Restore Orb Position if saved
            if let Some(pos) = storage::load_orb_position(app.handle()) {
                if let Some(orb_win) = app.get_webview_window("orb") {
                    let _ = orb_win.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: pos.x,
                        y: pos.y,
                    }));
                }
            }

            // Create System Tray Menu
            let show_hide_item = MenuItemBuilder::with_id("toggle", "Show/Hide J.A.R.V.I.S").build(app)?;
            let settings_item = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

            let tray_menu = MenuBuilder::new(app)
                .items(&[&show_hide_item, &settings_item, &quit_item])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().cloned().unwrap())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => {
                        let _ = commands::toggle_main_window(app.clone());
                    }
                    "settings" => {
                        if let Some(main_win) = app.get_webview_window("main") {
                            let _ = main_win.show();
                            let _ = main_win.set_focus();
                            let _ = main_win.emit("open-settings", ());
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let _ = commands::toggle_main_window(app.clone());
                    }
                })
                .build(app)?;

            // Register Global Shortcut: Ctrl+Shift+J
            let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyJ);
            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(shortcut, move |_app, _sc, event| {
                if event.state() == ShortcutState::Pressed {
                    let _ = commands::toggle_main_window(app_handle.clone());
                }
            })?;
            let _ = app.global_shortcut().register(shortcut);

            // Intercept Main Window close -> minimize to tray
            if let Some(main_win) = app.get_webview_window("main") {
                let main_win_clone = main_win.clone();
                main_win.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = main_win_clone.hide();
                    }
                });
            }

            // Start continuous background voice listener
            let voice_listener = VoiceListener::new();
            voice_listener.start(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
