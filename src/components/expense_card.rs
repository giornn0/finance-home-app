use crate::utils::{error_alert, get_headers, API_URL};
use core_api::error::AppError;
use core_api::models::ApiResponse;
use core_api::{icons::TrashIcon, models::Expense};
use gloo::dialogs::alert;
use leptos::error::Result;
use leptos::ev::MouseEvent;
use leptos::{component, spawn_local, view, IntoView};

async fn remove_post(expense: Expense) -> Result<()> {
    let resp = reqwasm::http::Request::delete(&format!("{}/expenses/{}", API_URL, expense.id))
        .headers(get_headers().await)
        .send()
        .await?;
    if resp.ok() {
        resp.json::<ApiResponse<bool>>().await?.data();
        return Ok(());
    }
    Err(AppError::HardError(resp.text().await?).into())
}

#[component]
pub fn ExpenseCard<F>(expense: Expense, fetch_expenses: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let remove_expense = move |e: MouseEvent, expense: Expense| {
        e.prevent_default();
        spawn_local(async move {
            match remove_post(expense).await {
                Ok(_) => {
                    alert("Successfully deleted!");
                    fetch_expenses();
                }
                Err(e) => error_alert(e),
            }
        });
    };

    view! {
    <li class="text-black row">
        <p>${expense.amount} - {expense.category_name.clone()} {" / "} {expense.description.clone()} - {expense.date.clone()} -</p>
        <TrashIcon on:click=move|e|{remove_expense(e,expense.clone());}  class=Some("text-red icon".to_owned()) />
    </li>}
}
