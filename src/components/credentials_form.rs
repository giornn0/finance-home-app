use crate::{
    app::invoke,
    utils::{error_alert, get_headers, API_URL},
};
use core_api::{
    error::AppError,
    models::{ApiResponse, LoginBody, LoginResponse, SetTokenArgs},
};
use leptos::{
    component, create_signal, error::Result, ev::SubmitEvent, event_target_value, logging,
    spawn_local, *,
};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

async fn post_login(credentials: LoginBody) -> Result<LoginResponse> {
    let value = JsValue::from_str(
        format!(
            r#"{{"email": "{}","password": "{}" }}"#,
            credentials.email, credentials.password
        )
        .as_str(),
    );

    let resp = reqwasm::http::Request::post(&format!("{}/login", API_URL))
        .headers(get_headers().await)
        .body(value)
        .send()
        .await?;
    if resp.ok() {
        let res = resp.json::<ApiResponse<LoginResponse>>().await?.data();
        let args = to_value(&SetTokenArgs {
            new_token: res.token.clone(),
        })
        .unwrap();
        match invoke("set_token", args).await.as_string() {
            Some(msg) => logging::debug_warn!("{msg}",),
            None => logging::debug_warn!("Error here -.-",),
        };
        return Ok(res);
    }
    Err(AppError::HardError(resp.text().await?).into())
}

#[component]
pub fn credentials_form<F>(refresh_data: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let (credentials, set_credentials) = create_signal(LoginBody::default());
    let update_email = move |ev| {
        let v = event_target_value(&ev);
        set_credentials.update(|credential| credential.email(v));
    };
    let update_password = move |ev| {
        let v = event_target_value(&ev);
        set_credentials.update(|credential| credential.password(v));
    };

    let reset_session = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let credentials = move || credentials.get_untracked();

            if credentials().password.is_empty() {
                return;
            }
            if credentials().email.is_empty() {
                return;
            }

            match post_login(credentials()).await {
                Ok(res) => {
                    logging::debug_warn!("RESPONSE POST: {:?}", res);
                    refresh_data();
                }
                Err(e) => error_alert(e),
            }
        });
    };

    view! {
            <form class="container" on:submit=reset_session>
               <div class="row" >
                    <input
                        id="email-input"
                        type="email"
                        placeholder="Enter your email"
                        on:input=update_email
                        prop:value={move||credentials.get().email}
                    />
               </div>
               <br />
               <div class="row" >
                    <input
                        id="password-input"
                        type="password"
                        placeholder="Enter your password."
                        on:input=update_password
                        prop:value={move||credentials.get().password}
                    />
               </div>
               <br />
               <button type="submit">"Reset session."</button>
            </form>

    }
}
