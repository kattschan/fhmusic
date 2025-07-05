use tauri::{Manager, menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent}};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn send_media_key(_key: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        use std::os::windows::process::CommandExt;
        
        // Use PowerShell with hidden window style to prevent foreground appearance
        let result = Command::new("powershell")
            .arg("-WindowStyle")
            .arg("Hidden")
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(format!(
                "Add-Type -AssemblyName System.Windows.Forms; \
                 $signature = '[DllImport(\"user32.dll\")] public static extern void keybd_event(byte bVk, byte bScan, uint dwFlags, uint dwExtraInfo);'; \
                 $SendInput = Add-Type -MemberDefinition $signature -Name \"Win32SendInput\" -Namespace Win32Functions -PassThru; \
                 $SendInput::keybd_event(0xB3, 0, 0, 0); \
                 Start-Sleep -Milliseconds 50; \
                 $SendInput::keybd_event(0xB3, 0, 2, 0);"
            ))
            .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
            .output();
            
        match result {
            Ok(output) => {
                if !output.status.success() {
                    return Err(format!("Failed to send media key: {}", String::from_utf8_lossy(&output.stderr)));
                }
            }
            Err(e) => {
                return Err(format!("Failed to execute media key command: {}", e));
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        return Err("Media key simulation is only supported on Windows".to_string());
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_udp::init())
        .invoke_handler(tauri::generate_handler![greet, send_media_key])
        .setup(|app| {
            // Create tray menu
            let open_config = MenuItem::with_id(app, "open_config", "Open Configuration", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_config, &quit])?;

            // Build tray icon
            let _tray = TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("FH Music Controller")
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } => {
                            // Left click - show/hide main window
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "open_config" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Prevent window from closing, hide it instead
                    api.prevent_close();
                    let _ = window.hide();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
