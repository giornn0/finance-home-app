use crate::components::{credentials_form::CredentialsForm, expense_form::ExpenseForm};
use crate::utils::{error_alert, get_headers, API_URL};
use core_api::icons::{BillIcon, ListIcon, RefreshIcon, UserIcon};
use core_api::models::{ApiResponse, Category, Expense};
use leptos::error::Result;
use leptos::*;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

use crate::components::expense_card::ExpenseCard;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "primitives"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
async fn fetch_all<T: Serialize + Clone + DeserializeOwned>(section: &str) -> Result<T> {
    Ok(
        reqwasm::http::Request::get(&format!("{}/{section}?all_listed=true", API_URL))
            .headers(get_headers().await)
            .send()
            .await?
            .json::<ApiResponse<T>>()
            .await?
            .data(),
    )
}

#[derive(Default, Clone)]
enum TabOpt {
    #[default]
    Session,
    NewExpense,
    AllExpenses,
}

#[component]
pub fn App() -> impl IntoView {
    let (expenses, set_expenses) = create_signal::<Vec<Expense>>(vec![]);
    let (current_tab, set_current_tab) = create_signal(TabOpt::default());
    let (categories, set_categories) = create_signal::<Vec<Category>>(vec![]);

    let fetch_categories = move || {
        spawn_local(async move {
            match fetch_all::<Vec<Category>>("categories").await {
                Ok(categories) => {
                    set_categories.set(categories);
                }
                Err(e) => error_alert(e),
            }
        });
    };

    let fetch_expenses = move || {
        spawn_local(async move {
            match fetch_all::<Vec<Expense>>("expenses").await {
                Ok(expenses) => {
                    set_expenses.set(expenses);
                }
                Err(e) => error_alert(e),
            }
        });
    };
    let refresh_data = move || {
        fetch_categories();
        fetch_expenses();
    };
    fetch_categories();
    fetch_expenses();

    let expenses_view = move || {
        expenses
            .get()
            .into_iter()
            .map(|expense| {
                view! {
                    <ExpenseCard expense={expense} fetch_expenses={fetch_expenses} />
                }
            })
            .collect_view()
    };

    let select_tab = move |tab: TabOpt| {
        set_current_tab.set(tab);
    };

    let app_content = move || match current_tab.get() {
        TabOpt::Session => {
            view! {
                <CredentialsForm refresh_data={refresh_data}/>
                <br />
            }
        }
        TabOpt::NewExpense => {
            view! {
            <div class="container" >
                <button on:click=move|_|{refresh_data()}>"Refetch"
                    <RefreshIcon  class=None/>
                </button>
            </div>
            <br />
            <ExpenseForm categories={categories} set_expenses={set_expenses}/>
            <br />
             }
        }
        TabOpt::AllExpenses => {
            view! {
            <div class="container" >
                <button on:click=move|_|{refresh_data()}>"Refetch"
                    <RefreshIcon  class=None/>
                </button>
            </div>
            <ul>
                {expenses_view}
            </ul>

            }
        }
    };

    // let test_func=move|e|{
    //     navi
    // };

    view! {
        <main>
            <div class="app-content">
            // <button on:click=test_func>Test</button>
                {app_content}
            </div>
            <nav>
                <ul>
                    <li class="nav-container" on:click=move|_|select_tab(TabOpt::Session)>
                        <UserIcon  class=None/>
                        Session
                    </li>
                    <li class="nav-container" on:click=move|_|select_tab(TabOpt::NewExpense)>
                        <BillIcon class=None/>
                        New Expense
                    </li>
                    <li class="nav-container" on:click=move|_|select_tab(TabOpt::AllExpenses)>
                        <ListIcon  class=None/>
                        All Expense
                    </li>
                </ul>
            </nav>
        </main>
    }
}
