use gloo::dialogs::alert;
use leptos::error::Error;
use reqwasm::http::Headers;
use wasm_bindgen::JsValue;

use crate::app::invoke;

pub fn error_alert(e: Error) {
    alert(format!("Error: {}", e).as_str());
}

pub const API_URL: &str = "https://arrebolit.com/apiFinance/api";

pub async fn get_headers() -> Headers {
    let token = invoke("get_token", JsValue::default())
        .await
        .as_string()
        .unwrap_or("no-token".to_owned());
    let headers = Headers::new();
    headers.append("Authorization", format!("Bearer {}", token).as_str());
    headers.append("content-type", "application/json");
    headers
}
