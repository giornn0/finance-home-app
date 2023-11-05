use core_api::{
    error::AppError,
    models::{ApiResponse, Category, Expense},
};
use leptos::{component, error::Result, ev::SubmitEvent, *};
use uuid::Uuid;
use wasm_bindgen::JsValue;

use crate::{
    app::invoke,
    utils::{error_alert, get_headers, API_URL},
};

async fn post_expense(expense: Expense) -> Result<Expense> {
    let value = JsValue::from_str(
        format!(
            r#"{{"category_id": "{}","date": "{}","amount":{}, "description":"{}" }}"#,
            expense.category_id,
            expense.date.clone().unwrap_or(String::default()),
            expense.amount,
            expense.description.unwrap_or(String::default())
        )
        .as_str(),
    );

    let resp = reqwasm::http::Request::post(&format!("{}/expenses", API_URL))
        .headers(get_headers().await)
        .body(value)
        .send()
        .await?;
    if resp.ok() {
        return Ok(resp.json::<ApiResponse<Expense>>().await?.data());
    }
    Err(AppError::HardError(resp.text().await?).into())
}

#[component]
pub fn expense_form(
    categories: ReadSignal<Vec<Category>>,
    set_expenses: WriteSignal<Vec<Expense>>,
) -> impl IntoView {
    let (new_expense, set_new_expense) = create_signal(Expense::default());
    let update_description = move |ev| {
        let v = event_target_value(&ev);
        set_new_expense.update(|expense| expense.description(Some(v)));
    };
    let update_amount = move |ev| {
        let v = event_target_value(&ev)
            .parse()
            .unwrap_or(new_expense.get().amount);
        set_new_expense.update(|expense| expense.amount(v));
    };
    let update_date = move |ev| {
        let v = event_target_value(&ev);
        set_new_expense.update(|expense| expense.date(v));
    };
    let update_category_id = move |ev| {
        let v = event_target_value(&ev);
        if v.is_empty() {
            return;
        }
        let v = Uuid::parse_str(v.as_str()).unwrap();
        let category_selected = categories
            .get_untracked()
            .into_iter()
            .find(|category| category.id == v)
            .unwrap();

        set_new_expense.update(|expense| expense.category(v, category_selected.name));
    };

    let get_now = move || {
        spawn_local(async move {
            let now = invoke("get_now_date", JsValue::default())
                .await
                .as_string()
                .unwrap();
            set_new_expense.update(|expense| expense.date(now));
        });
    };

    let add_expense = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let expense = move || new_expense.get_untracked();
            if expense().amount <= 1 {
                return;
            }
            if expense().date.is_none() || expense().date.unwrap().is_empty() {
                return;
            }
            if expense().category_id == Uuid::default() {
                return;
            }

            match post_expense(expense()).await {
                Ok(res) => {
                    set_expenses.update(|expenses_listed| expenses_listed.push(res));
                }
                Err(e) => error_alert(e),
            }
        });
    };
    let categories_view = move || {
        categories
            .get()
            .iter()
            .map(|category| {
                view! {
                <option class="text-black" value={category.id.clone().to_string()}>
                    <p>{category.name.clone()}</p>
                </option>}
            })
            .collect_view()
    };

    get_now();
    view! {
                <form class="container" on:submit=add_expense>
                   <div class="row" >
                        <input
                            id="amount-input"
                            type="number"
                            placeholder="Enter an amount"
                            on:input=update_amount
                            prop:value={move||new_expense.get().amount}
                        />
                   </div>
                   <br />
                   <div class="row" >
                        <input
                            id="date-input"
                            type="date"
                            placeholder="Enter a date"
                            on:input=update_date
                            prop:value={move||new_expense.get().date.unwrap_or("Enter a Date.".to_owned())}
                        />
                   </div>
                   <br />
                       <select
                            on:change=update_category_id
                            prop:value={move||new_expense.get().category_id.to_string()}
                            >
                            <option value={Some(Uuid::default().to_string())}>Select category</option>
                            {categories_view}
                       </select>
                   <br />
                   <div class="row" >
                        <input
                            id="description-input"
                            placeholder="Enter a description"
                            on:input=update_description
                        />
                   </div>
                   <br />
                   <button type="submit">"Add Expense"</button>
                </form>

    }
}
