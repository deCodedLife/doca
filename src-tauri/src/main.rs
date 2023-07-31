// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod doca;
pub mod ui;
mod atol;

#[macro_use]
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;
use tokio::main;

fn doca_exists() -> bool {
    let sys = sysinfo::System::new_all();
    let mut process_list = sys.processes_by_name( "DocaCRM" );
    process_list.count() != 1
}

fn app_exit() {
    println!( "[-] Process already exists" );
    std::process::exit(-1);
}

#[tokio::main]
async fn main() {
    doca_exists()
        .then(app_exit);
    doca::init();

    let restore = CustomMenuItem::new("restore".to_string(), "Развернуть");
    let restart = CustomMenuItem::new("restart".to_string(), "Перезапустить");
    let exit = CustomMenuItem::new("exit".to_string(), "Завершить");
    let system_tray = SystemTrayMenu::new()
        .add_item(restore)
        .add_item(restart)
        .add_item(exit);

    tauri::Builder::default()
        .system_tray( SystemTray::new().with_menu(system_tray) )
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "restore" => { app.get_window("main").unwrap().show().unwrap() }
                    "exit" => { app.exit(0) }
                    "restart" => { app.restart() }
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
            ui::get_cashbox_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
