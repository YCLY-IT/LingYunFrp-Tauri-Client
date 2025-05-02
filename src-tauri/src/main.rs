// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::Runtime;
use tauri::Emitter;
use tauri::command;

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
            hide_to_tray,
            check_frpc_exists,
            check_update,
            emit_event,
            get_app_data_dir,
            open_app_data_dir,
            get_frpc_cli_version,
            download_frpc,
            toggle_auto_start,
            kill_all_processes,
            get_cpl_version
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

#[tauri::command]
fn check_frpc_exists(app: tauri::AppHandle) -> bool {
    // 1. 检查应用数据目录
    let app_data_dir = app.path().app_data_dir().unwrap();
    let frpc_path = app_data_dir.join("frpc.exe");
    println!("检查路径1: {:?}", frpc_path);
    
    if frpc_path.exists() {
        return true;
    }

    // 2. 检查程序所在目录
    if let Ok(exe_dir) = std::env::current_exe() {
        let exe_path = exe_dir.parent().unwrap().join("frpc.exe");
        println!("检查路径2: {:?}", exe_path);
        if exe_path.exists() {
            return true;
        }
    }

    false
}
const API_URL: &str = "http://localhost:8081/";
const VERSION: &str = "0.0.1";

#[tauri::command]
async fn check_update(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let current_version = VERSION;
    let response = reqwest::get(API_URL.to_string() + "frp/updates/latest")
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "code": 0,
        "message": null,
        "data": {
            "current_version": current_version,
            "latest_info": response,
            "needs_update": response["version"].as_str() != Some(current_version)
        }
    }))
}

#[command]
async fn emit_event<R: Runtime>(
    app: tauri::AppHandle<R>,
    event: String,
    payload: serde_json::Value,
) -> Result<(), String> {
    app.emit(&event, payload)
        .map_err(|e| format!("发送事件失败: {}", e))
}

#[tauri::command]
async fn get_app_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let app_data_dir_str = app_data_dir.to_str().ok_or("路径转换失败")?.to_string();
    Ok(app_data_dir_str)
}

#[tauri::command]
async fn open_app_data_dir(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    std::process::Command::new("explorer")
        .arg(app_data_dir)
        .spawn()
        .map_err(|e| format!("打开目录失败: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn download_frpc(app: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join("frpc.exe");
    if frpc_path.exists() {
        return Err("frpc.exe已存在".to_string());
    }
    let response = reqwest::get(API_URL.to_string() + "frp/download")
       .await
       .map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(frpc_path, bytes).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_frpc_cli_version(app: tauri::AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|_| "无法获取应用数据目录")?;
    let frpc_path = app_data_dir.join("frpc.exe");
    if !frpc_path.exists() {
        return Err("frpc.exe不存在".to_string());
    }
    
    let output = std::process::Command::new(frpc_path)
        .arg("--version")
        .output()
        .map_err(|e| format!("获取版本失败: {}", e))?;
    
    let version_str = String::from_utf8_lossy(&output.stdout);
    Ok(version_str.trim().to_string())
}

#[tauri::command]
async fn toggle_auto_start(_enable: bool) -> Result<(), String> {
    // TODO: 实现开机自启动设置
    Ok(())
}

#[tauri::command]
fn get_cpl_version() -> String {
    VERSION.to_string()
}

#[tauri::command]
async fn kill_all_processes() -> Result<(), String> {
    // 终止所有frpc进程
    
        let output = std::process::Command::new("taskkill")
            .arg("/F")
            .arg("/IM")
            .arg("frpc.exe")
            .output()
            .map_err(|e| format!("终止进程失败: {}", e))?;
        if !output.status.success() {
            return Err("终止进程失败".to_string());
        }
        Ok(())
}
