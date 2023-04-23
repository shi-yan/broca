#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::Position;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
use tokio::runtime::Runtime;

use std::rc::Rc;
mod entry;
mod openai;
mod state;
mod win_ext;
use win_ext::WindowExt;

#[tauri::command]
fn load_config(state: tauri::State<Mutex<state::State>>) -> Result<String, String> {
    if let Ok(content) = state.lock().unwrap().load_config() {
        return Ok(content);
    }

    Err("Failed to load config".to_string())
}

#[tauri::command]
fn first_time_setup(
    state: tauri::State<Mutex<state::State>>,
    workspace_path: &str,
    openai_token: &str,
) -> Result<String, String> {
    println!("{} {}", workspace_path, openai_token);
    if let Ok(content) = state
        .lock()
        .unwrap()
        .first_time_setup(workspace_path, openai_token)
    {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn scan_vocabulary(state: tauri::State<Mutex<state::State>>) -> Result<Vec<String>, String> {
    if let Ok(content) = state.lock().unwrap().scan_vocabulary() {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn load_word(state: tauri::State<Mutex<state::State>>, query: &str) -> Result<String, String> {
    if let Ok(content) = state.lock().unwrap().load_word(query) {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn query_words(
    state: tauri::State<Mutex<state::State>>,
    query: &str,
) -> Result<Vec<String>, String> {
    if let Ok(content) = state.lock().unwrap().query_words(query) {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn search(state: tauri::State<Mutex<state::State>>, query: &str) -> Result<String, String> {
    let rt  = Runtime::new().unwrap();

   let r = rt.block_on(async {
        return state.lock().unwrap().search(query).await;
    });
    //println!("{}",r.unwrap());
    return Ok(r.unwrap());
    /*if let Ok(content) = state.lock().unwrap().search(query).await {
        return Ok(Rc::new(content.into()));
    }*/

    //Err("Can't initialize workspace.".to_string())
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
        .invoke_handler(tauri::generate_handler![
            greet,
            load_config,
            first_time_setup,
            scan_vocabulary,
            load_word,
            query_words,
            search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
