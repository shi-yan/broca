#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::sync::{Arc, RwLock};
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
fn load_config(state: tauri::State<Arc<RwLock<state::State>>>) -> Result<state::Config, String> {
    match state.write().unwrap().load_config() {
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
    state: tauri::State<Arc<RwLock<state::State>>>,
    workspace_path: &str,
    openai_token: &str,
    target_lang: &str,
    aws_key: Option<&str>,
    aws_secret: Option<&str>,
) -> Result<state::Config, String> {
    println!(
        "{} {} {} {:?} {:?}",
        workspace_path, openai_token, target_lang, aws_key, aws_secret
    );
    match state.write().unwrap().first_time_setup(
        workspace_path,
        openai_token,
        target_lang,
        aws_key,
        aws_secret,
    ) {
        Ok(content) => return Ok(content),
        Err(message) => return Err(message.to_string()),
    }
}

#[tauri::command]
fn scan_vocabulary(state: tauri::State<Arc<RwLock<state::State>>>) -> Result<Vec<String>, String> {
    match state.read().unwrap().scan_vocabulary() {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn load_word(
    state: tauri::State<Arc<RwLock<state::State>>>,
    query: &str,
) -> Result<String, String> {
    match state.read().unwrap().load_word(query) {
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
    state: tauri::State<Arc<RwLock<state::State>>>,
    query: &str,
) -> Result<Vec<String>, String> {
    match state.read().unwrap().query_words(query) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn search(state: tauri::State<Arc<RwLock<state::State>>>, query: &str) -> Result<String, String> {
    let rt = Runtime::new().unwrap();

    match rt.block_on(state.read().unwrap().search(query)) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn say(state: tauri::State<Arc<RwLock<state::State>>>, query: &str) -> Result<String, String> {
    let rt = Runtime::new().unwrap();

    match rt.block_on(state.read().unwrap().say(query)) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn delete_word(
    state: tauri::State<Arc<RwLock<state::State>>>,
    query: &str,
) -> Result<String, String> {
    if let Ok(content) = state.read().unwrap().delete_word(query) {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn fetch_all_words(state: tauri::State<Arc<RwLock<state::State>>>) -> Result<Vec<String>, String> {
    if let Ok(content) = state.read().unwrap().fetch_all_words() {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
fn generate_more_examples(
    state: tauri::State<'_, Arc<RwLock<state::State>>>,
    entry: &str,
    meaning: &str,
) -> Result<String, String> {
    let rt = Runtime::new().unwrap();

    match rt.block_on(
        state
            .write()
            .unwrap()
            .search_example_sentences(entry, meaning)) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
fn load_usage(
    state: tauri::State<'_, Arc<RwLock<state::State>>>
) -> Result<[i64;2], String> {
    if let Ok(content) = state.read().unwrap().load_usage() {
        return Ok(content);
    }
    Err("Can't load usage.".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::<RwLock<state::State>>::new(RwLock::new(
            state::State::new(),
        )))
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
            fetch_all_words,
            say,
            generate_more_examples,
            load_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
