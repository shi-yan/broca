#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::Position;
use std::sync::Mutex;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod openai;
mod state;
mod entry;
mod win_ext;
use win_ext::WindowExt;

#[tauri::command]
fn load_config(state: tauri::State<Mutex<state::State>>) -> Result<state::WorkspaceContent, String> {
    println!("inside load config");
    if let Ok(content) =state.lock().unwrap().load_config() {
        return Ok(content);
    }

    Err("Failed to load config".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::<state::State>::new(state::State::new()))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            // window.open_devtools();
            window.set_transparent_titlebar(true);
            //window.maximize().unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
