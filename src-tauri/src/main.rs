#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Child};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use tauri::menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem};

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

struct TrayState<R: tauri::Runtime> {
    tray: tauri::tray::TrayIcon<R>,
    start_item: tauri::menu::MenuItem<R>,
    stop_item: tauri::menu::MenuItem<R>,
    status_item: tauri::menu::MenuItem<R>,
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

fn update_tray_menu<R: tauri::Runtime>(app: &AppHandle<R>, is_running: bool) {
    if let Some(tray_state) = app.try_state::<Mutex<TrayState<R>>>() {
        let tray_state = tray_state.lock().unwrap();
        
        // 更新菜单项状态
        let _ = tray_state.start_item.set_enabled(!is_running);
        let _ = tray_state.stop_item.set_enabled(is_running);
        
        // 更新状态显示
        let status_text = if is_running {
            "● 服务运行中"
        } else {
            "○ 服务已停止"
        };
        let _ = tray_state.status_item.set_text(status_text);
        
        // 更新托盘提示
        let tooltip = if is_running {
            "EasyCue - 运行中"
        } else {
            "EasyCue - 已停止"
        };
        let _ = tray_state.tray.set_tooltip(Some(tooltip));
    }
}

fn main() {
    let service_state = Mutex::new(ServiceState {
        process: None,
        status: ServiceStatus::Stopped,
    });
    
    tauri::Builder::default()
        .manage(service_state)
        .setup(|app| {
            // 隐藏dock图标（macOS）
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                let _ = app.set_activation_policy(ActivationPolicy::Accessory);
            }
            
            // 创建菜单项
            let status_item = MenuItemBuilder::with_id("status", "○ 服务已停止")
                .enabled(false)
                .build(app)?;
            
            let separator1 = PredefinedMenuItem::separator(app)?;
            
            let start_item = MenuItemBuilder::with_id("start", "▶ 启动服务")
                .enabled(true)
                .build(app)?;
            
            let stop_item = MenuItemBuilder::with_id("stop", "■ 停止服务")
                .enabled(false)
                .build(app)?;
            
            let separator2 = PredefinedMenuItem::separator(app)?;
            
            let copy_item = MenuItemBuilder::with_id("copy", "复制服务地址")
                .build(app)?;
            
            let separator3 = PredefinedMenuItem::separator(app)?;
            
            let about_item = MenuItemBuilder::with_id("about", "关于 EasyCue")
                .build(app)?;
            
            let quit_item = MenuItemBuilder::with_id("quit", "退出")
                .build(app)?;
            
            // 构建菜单
            let menu = MenuBuilder::new(app)
                .items(&[
                    &status_item,
                    &separator1,
                    &start_item,
                    &stop_item,
                    &separator2,
                    &copy_item,
                    &separator3,
                    &about_item,
                    &quit_item,
                ])
                .build()?;
            
            // 获取配置文件创建的托盘
            let tray = app.tray_by_id("main")
                .expect("Failed to get tray icon");
            
            // 设置图标（确保显示）
            let icon = tauri::image::Image::from_bytes(include_bytes!("../icons/icon.png"))
                .expect("Failed to load icon");
            tray.set_icon(Some(icon))?;
            
            // 设置菜单和事件处理
            tray.set_menu(Some(menu.clone()))?;
            tray.set_tooltip(Some("EasyCue - 已停止"))?;
            tray.on_menu_event(move |app, event| {
                    let service_state = app.state::<Mutex<ServiceState>>();
                    
                    match event.id.as_ref() {
                        "start" => {
                            let mut service = service_state.lock().unwrap();
                            if !matches!(service.status, ServiceStatus::Running) {
                                drop(service);
                                if let Ok(_) = start_service(service_state.clone()) {
                                    update_tray_menu(app, true);
                                }
                            }
                        }
                        "stop" => {
                            let mut service = service_state.lock().unwrap();
                            if matches!(service.status, ServiceStatus::Running) {
                                drop(service);
                                if let Ok(_) = stop_service(service_state.clone()) {
                                    update_tray_menu(app, false);
                                }
                            }
                        }
                        "copy" => {
                            // 复制到剪贴板
                            #[cfg(target_os = "macos")]
                            {
                                let _ = Command::new("sh")
                                    .arg("-c")
                                    .arg("echo 'http://localhost:8080' | pbcopy")
                                    .spawn();
                            }
                        }
                        "about" => {
                            // 可以显示一个关于对话框
                            println!("EasyCue v0.1.0 - PromptX Desktop Client");
                        }
                        "quit" => {
                            let _ = stop_service(service_state.clone());
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                });
            
            // 保存托盘状态以便后续更新
            app.manage(Mutex::new(TrayState {
                tray: app.tray_by_id("main").unwrap(),
                start_item,
                stop_item,
                status_item,
            }));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}