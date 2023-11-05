use std::sync::{Arc, Mutex};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::State;
use time::{format_description, OffsetDateTime};

#[tauri::command]
fn get_now_date() -> String {
    let now_odt = OffsetDateTime::now_utc();
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    now_odt.date().format(&format).unwrap()
}

#[tauri::command]
fn get_token(token: State<'_, Token>) -> String {
    token.0.lock().unwrap().to_string()
}
#[tauri::command(rename_all = "snake_case")]
fn set_token(new_token: String, token: State<'_, Token>) -> String {
    *token.0.lock().unwrap() = new_token;
    "Successfully updated!".to_owned()
}

#[derive(Default)]
struct Token(Arc<Mutex<String>>);
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Token(Default::default()))
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_now_date, get_token, set_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
