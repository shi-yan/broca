#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::Position;
use tokio::runtime::Runtime;

use std::rc::Rc;
mod entry;
mod openai;
mod state;
mod win_ext;
use win_ext::WindowExt;

#[tauri::command]
fn load_config(state: tauri::State<Mutex<state::State>>) -> Result<String, String> {
    match state.lock().unwrap().load_config() {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn first_time_setup(
    state: tauri::State<Mutex<state::State>>,
    workspace_path: &str,
    openai_token: &str,
    target_lang: &str
) -> Result<String, String> {
    println!("{} {} {}", workspace_path, openai_token, target_lang);
    match state
        .lock()
        .unwrap()
        .first_time_setup(workspace_path, openai_token, target_lang)
    {
        Ok(content) => return Ok(content),
        Err(message) => return Err(message.to_string()),
    }
}

#[tauri::command]
fn scan_vocabulary(state: tauri::State<Mutex<state::State>>) -> Result<Vec<String>, String> {
    match state.lock().unwrap().scan_vocabulary() {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn load_word(state: tauri::State<Mutex<state::State>>, query: &str) -> Result<String, String> {
    match state.lock().unwrap().load_word(query) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn query_words(
    state: tauri::State<Mutex<state::State>>,
    query: &str,
) -> Result<Vec<String>, String> {
    match state.lock().unwrap().query_words(query) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn search(state: tauri::State<Mutex<state::State>>, query: &str) -> Result<String, String> {
    let rt = Runtime::new().unwrap();

    match  rt.block_on( state.lock().unwrap().search(query)) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn delete_word(state: tauri::State<Mutex<state::State>>, query: &str) -> Result<String, String> {
    if let Ok(content) = state.lock().unwrap().delete_word(query) {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn fetch_all_words(state: tauri::State<Mutex<state::State>>) -> Result<Vec<String>, String> {
    if let Ok(content) = state.lock().unwrap().fetch_all_words() {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
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
            load_config,
            first_time_setup,
            scan_vocabulary,
            load_word,
            query_words,
            search,
            delete_word,
            fetch_all_words
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
