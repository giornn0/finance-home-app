pub mod error {
    use std::fmt::Display;
    use thiserror::Error;

    #[derive(Error, Clone, Debug)]
    pub enum AppError<'a> {
        Invalid(&'a str),
        HardError(String),
    }

    impl<'a> Display for AppError<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}

pub mod models {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Expense {
        pub id: Uuid,
        pub category_id: Uuid,
        pub category_name: String,
        pub description: Option<String>,
        pub amount: i32,
        pub date: Option<String>,
    }
    impl Default for Expense {
        fn default() -> Expense {
            Expense {
                id: Uuid::default(),
                category_id: Uuid::default(),
                description: None,
                amount: i32::default(),
                category_name: String::default(),
                date: None,
            }
        }
    }
    impl Expense {
        pub fn new() -> Self {
            Expense::default()
        }
        pub fn amount(&mut self, amount: i32) {
            self.amount = amount;
        }

        pub fn description(&mut self, description: Option<String>) {
            self.description = description;
        }
        pub fn category(&mut self, category_id: Uuid, category_name: String) {
            self.category_id = category_id;
            self.category_name = category_name;
        }
        pub fn date(&mut self, date: String) {
            self.date = Some(date)
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct Category {
        pub id: Uuid,
        pub name: String,
        description: Option<String>,
    }
    impl Category {
        pub fn new() -> Self {
            let mut category = Category::default();
            category.id = Uuid::new_v4();
            category
        }
        pub fn name(self, name: String) -> Self {
            Category { name, ..self }
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct LoginBody {
        pub email: String,
        pub password: String,
    }
    impl LoginBody {
        pub fn new() -> Self {
            LoginBody::default()
        }
        pub fn email(&mut self, email: String) {
            self.email = email;
        }
        pub fn password(&mut self, password: String) {
            self.password = password;
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct SetTokenArgs {
        pub new_token: String,
    }
    #[derive(Debug, Serialize, Deserialize, Clone, Default)]
    pub struct LoginResponse {
        pub token: String,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ApiResponse<T: Serialize + Clone> {
        data: T,
        errors: Option<Vec<String>>,
        success: bool,
    }
    impl<T: Serialize + Clone> ApiResponse<T> {
        pub fn data(self) -> T {
            self.data
        }
    }
}

pub mod icons {
    use leptos::*;
    #[component]
    pub fn trash_icon(class: Option<String>) -> impl IntoView {
        view! {
           <svg  class={class} width="1em" height="1em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16l-1.58 14.22A2 2 0 0 1 16.432 22H7.568a2 2 0 0 1-1.988-1.78L4 6Zm3.345-2.853A2 2 0 0 1 9.154 2h5.692a2 2 0 0 1 1.81 1.147L18 6H6l1.345-2.853ZM2 6h20m-12 5v5m4-5v5"></path></svg>
        }
    }
    #[component]
    pub fn refresh_icon(class: Option<String>) -> impl IntoView {
        view! {
            <svg  class={class} width="1em" height="1em" viewBox="0 0 24 24"><g fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"><path d="M4.266 16.06a8.923 8.923 0 0 0 3.915 3.978a8.706 8.706 0 0 0 5.471.832a8.796 8.796 0 0 0 4.887-2.64a9.067 9.067 0 0 0 2.388-5.079a9.137 9.137 0 0 0-1.044-5.53a8.904 8.904 0 0 0-4.068-3.815a8.7 8.7 0 0 0-5.5-.608c-1.85.401-3.367 1.313-4.62 2.755a7.62 7.62 0 0 0-1.22 1.781"></path><path d="m8.931 7.813l-5.04.907L3 3.59"></path></g></svg>
        }
    }
    #[component]
    pub fn bill_icon(class: Option<String>) -> impl IntoView {
        view! {
           <svg class={class} width="1em" height="1em" viewBox="0 0 24 24"><g fill="none"><g fill="currentColor" clipPath="url(#akarIconsBitcoinFill0)"><path d="M11.385 15.275c1.111-.004 3.54-.013 3.512-1.558c-.027-1.58-2.36-1.485-3.497-1.438c-.127.005-.24.01-.332.011l.052 2.987c.075-.002.165-.002.265-.002Zm-.118-4.353c.927-.001 2.95-.003 2.926-1.408c-.026-1.437-1.969-1.352-2.918-1.31c-.107.005-.2.009-.278.01l.047 2.709l.223-.001Z"></path><path fillRule="evenodd" d="M9.096 23.641c6.43 1.603 12.942-2.31 14.545-8.738C25.244 8.474 21.33 1.962 14.9.36C8.474-1.244 1.962 2.67.36 9.1c-1.603 6.428 2.31 12.94 8.737 14.542Zm4.282-17.02c1.754.124 3.15.638 3.333 2.242c.136 1.174-.344 1.889-1.123 2.303c1.3.288 2.125 1.043 1.995 2.771c-.161 2.145-1.748 2.748-4.026 2.918l.038 2.25l-1.356.025l-.039-2.22c-.351.006-.711.01-1.084.008l.04 2.23l-1.356.024l-.04-2.254l-.383.003c-.194.001-.39.002-.586.006l-1.766.03l.241-1.624s1.004-.002.986-.017c.384-.008.481-.285.502-.459L8.693 11.3l.097-.002h.046a1.101 1.101 0 0 0-.144-.007l-.044-2.54c-.057-.274-.241-.59-.79-.58c.015-.02-.986.017-.986.017L6.846 6.74l1.872-.032v.007c.281-.005.57-.015.863-.026L9.543 4.46l1.356-.023l.038 2.184c.362-.013.726-.027 1.083-.033l-.038-2.17l1.357-.024l.039 2.229Z" clipRule="evenodd"></path></g><defs><clipPath id="akarIconsBitcoinFill0"><path fill="#fff" d="M0 0h24v24H0z"></path></clipPath></defs></g></svg>
        }
    }
    #[component]
    pub fn list_icon(class: Option<String>) -> impl IntoView {
        view! {
           <svg class={class} width="1em" height="1em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="2" d="M3 6h18M3 12h18M3 18h18"></path></svg>
        }
    }
    #[component]
    pub fn user_icon(class: Option<String>) -> impl IntoView {
        view! {
        <svg class={class} width="1em" height="1em" viewBox="0 0 24 24"><g fill="none" stroke="currentColor" strokeWidth="2"><circle cx="12" cy="7" r="5"></circle><path strokeLinecap="round" strokeLinejoin="round" d="M17 14h.352a3 3 0 0 1 2.976 2.628l.391 3.124A2 2 0 0 1 18.734 22H5.266a2 2 0 0 1-1.985-2.248l.39-3.124A3 3 0 0 1 6.649 14H7"></path></g></svg>        }
    }
}
