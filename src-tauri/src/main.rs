#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use futures::lock::Mutex;
use tauri::Manager;

mod entry;
mod openai;
mod state;
mod win_ext;
use win_ext::WindowExt;

#[tauri::command]
async fn load_config(
    state: tauri::State<'_, Mutex<state::State>>,
) -> Result<state::Config, String> {
    match state.lock().await.load_config() {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn first_time_setup(
    state: tauri::State<'_, Mutex<state::State>>,
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
    match state.lock().await.first_time_setup(
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
async fn scan_vocabulary(
    state: tauri::State<'_, Mutex<state::State>>,
) -> Result<Vec<String>, String> {
    match state.lock().await.scan_vocabulary() {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn load_word(
    state: tauri::State<'_, Mutex<state::State>>,
    query: &str,
) -> Result<String, String> {
    match state.lock().await.load_word(query) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn query_words(
    state: tauri::State<'_, Mutex<state::State>>,
    query: &str,
) -> Result<Vec<String>, String> {
    match state.lock().await.query_words(query) {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn search(
    state: tauri::State<'_, Mutex<state::State>>,
    query: &str,
) -> Result<String, String> {
    match state.lock().await.search(query).await {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn say(state: tauri::State<'_, Mutex<state::State>>, query: &str) -> Result<String, String> {
    match state.lock().await.say(query).await {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn delete_word(
    state: tauri::State<'_, Mutex<state::State>>,
    query: &str,
) -> Result<String, String> {
    if let Ok(content) = state.lock().await.delete_word(query) {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
async fn fetch_all_words(
    state: tauri::State<'_, Mutex<state::State>>,
) -> Result<Vec<String>, String> {
    if let Ok(content) = state.lock().await.fetch_all_words() {
        return Ok(content);
    }
    Err("Can't initialize workspace.".to_string())
}

#[tauri::command]
async fn generate_more_examples(
    state: tauri::State<'_, Mutex<state::State>>,
    entry: &str,
    meaning: &str,
) -> Result<String, String> {
    match state
        .lock()
        .await
        .search_example_sentences(entry, meaning)
        .await
    {
        Ok(content) => {
            return Ok(content);
        }
        Err(message) => {
            return Err(message.to_string());
        }
    }
}

#[tauri::command]
async fn load_usage(state: tauri::State<'_, Mutex<state::State>>) -> Result<[i64; 2], String> {
    if let Ok(content) = state.lock().await.load_usage() {
        return Ok(content);
    }
    Err("Can't load usage.".to_string())
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
            fetch_all_words,
            say,
            generate_more_examples,
            load_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
