// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};

#[tauri::command]
fn close_window(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command]
fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn toggle_maximize(window: tauri::Window) {
    let is_maximized = window.is_maximized().unwrap();
    if is_maximized {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn hide_to_tray(window: tauri::Window) {
    window.hide().unwrap();
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_window,
            minimize_window,
            toggle_maximize,
            hide_to_tray
        ])
        .setup(|app| {
            create_tray_menu(app)?;
            let window = app.get_webview_window("main").unwrap();
            window.set_decorations(false).unwrap();
            // 添加拖动区域
            window.on_window_event(|event| {
                if let tauri::WindowEvent::Moved { .. } = event {
                    // 处理拖动事件

                }
            });

            #[cfg(target_os = "windows")]
            window.set_ignore_cursor_events(false).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error in running tauri application");
}

fn create_tray_menu(app: &tauri::App) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?,
            &MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?,
        ],
    )?;

    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(tray)
}
