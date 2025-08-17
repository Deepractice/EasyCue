#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Child};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

#[derive(Clone, serde::Serialize)]
enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error(String),
}

struct ServiceState {
    process: Option<Child>,
    status: ServiceStatus,
}

#[tauri::command]
fn start_service(state: State<Mutex<ServiceState>>) -> Result<String, String> {
    let mut service = state.lock().unwrap();
    
    if matches!(service.status, ServiceStatus::Running) {
        return Ok("Already running".to_string());
    }
    
    service.status = ServiceStatus::Starting;
    
    // Mock service - just runs a sleep command
    #[cfg(unix)]
    let cmd = Command::new("sleep").arg("3600").spawn();
    
    #[cfg(windows)]
    let cmd = Command::new("cmd").args(&["/C", "timeout", "/T", "3600"]).spawn();
    
    match cmd {
        Ok(child) => {
            service.process = Some(child);
            service.status = ServiceStatus::Running;
            Ok("Service started".to_string())
        }
        Err(e) => {
            service.status = ServiceStatus::Error(e.to_string());
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn stop_service(state: State<Mutex<ServiceState>>) -> Result<String, String> {
    let mut service = state.lock().unwrap();
    
    if let Some(mut child) = service.process.take() {
        service.status = ServiceStatus::Stopping;
        child.kill().map_err(|e| e.to_string())?;
        service.status = ServiceStatus::Stopped;
        Ok("Service stopped".to_string())
    } else {
        Ok("Service not running".to_string())
    }
}

#[tauri::command]
fn get_status(state: State<Mutex<ServiceState>>) -> ServiceStatus {
    state.lock().unwrap().status.clone()
}

#[tauri::command]
fn copy_address(_app: AppHandle) -> Result<String, String> {
    // Mock address for now
    let address = "http://localhost:8080";
    Ok(address.to_string())
}

fn main() {
    let service_state = Mutex::new(ServiceState {
        process: None,
        status: ServiceStatus::Stopped,
    });
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(service_state)
        .setup(|app| {
            // Create tray icon in setup
            use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState};
            use tauri::menu::{MenuBuilder, MenuItemBuilder};
            use tauri::image::Image;
            
            let toggle = MenuItemBuilder::with_id("toggle", "启动/停止").build(app)?;
            let copy = MenuItemBuilder::with_id("copy", "复制地址").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            
            let menu = MenuBuilder::new(app)
                .items(&[&toggle, &copy, &quit])
                .build()?;
            
            let _ = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("EasyCue - PromptX Client")
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "toggle" => {
                            let state = app.state::<Mutex<ServiceState>>();
                            let service = state.lock().unwrap();
                            
                            if matches!(service.status, ServiceStatus::Running) {
                                drop(service);
                                let _ = stop_service(state);
                            } else {
                                drop(service);
                                let _ = start_service(state);
                            }
                        }
                        "copy" => {
                            let _ = copy_address(app.clone());
                        }
                        "quit" => {
                            let state = app.state::<Mutex<ServiceState>>();
                            let _ = stop_service(state);
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    use tauri::tray::TrayIconEvent;
                    
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        let state = app.state::<Mutex<ServiceState>>();
                        let service = state.lock().unwrap();
                        
                        if matches!(service.status, ServiceStatus::Running) {
                            drop(service);
                            let _ = stop_service(state);
                        } else {
                            drop(service);
                            let _ = start_service(state);
                        }
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_service,
            stop_service,
            get_status,
            copy_address
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}