use leptos::{component, IntoView, view};

#[component]
pub fn player(col: i32, row: i32) -> impl IntoView {
    view! { <div>Player {col} {row}</div> }
}