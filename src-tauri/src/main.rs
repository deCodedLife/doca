// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod doca;
pub mod ui;
mod atol;
mod configs;
mod autostart;

use sysinfo::SystemExt;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager};

fn doca_exists() -> bool {
    let sys = sysinfo::System::new_all();
    let process_list = sys.processes_by_name( "DocaCRM" );
    process_list.count() != 1
}

fn app_exit() {
    println!( "[-] Process already exists" );
    std::process::exit(-1);
}

fn autostart(cfg: &configs::Config) {
    let is_configured = autostart::is_configured().unwrap_or(false);
    if cfg.autorun && !is_configured {
        autostart::add_to_autostart()
            .expect("[-] Can't add to autostart ");
    }
    if !cfg.autorun && is_configured {
        autostart::remove_from_autostart()
            .expect("[-] Can't remove from autostart ");
    }
}


#[tokio::main]
async fn main() {
    doca_exists()
        .then(app_exit);

    let cfg = configs::get_config();
    doca::init(&cfg);

    #[cfg(target_os = "windows")]
    autostart(&cfg);

    #[cfg(target_os = "macos")]
    autostart(&cfg);

    let restore = CustomMenuItem::new("restore".to_string(), "Развернуть");
    let exit = CustomMenuItem::new("exit".to_string(), "Завершить");
    let system_tray = SystemTrayMenu::new()
        .add_item(restore)
        .add_item(exit);

    tauri::Builder::default()
        .manage(ui::ConfigState(cfg))
        .system_tray( SystemTray::new().with_menu(system_tray) )
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "restore" => { app.get_window("main").unwrap().show().unwrap() }
                    "exit" => { app.exit(0) }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
                tauri::WindowEvent::CloseRequested { api, ..} => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            ui::get_crm_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
