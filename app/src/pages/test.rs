use leptos::{component, view, IntoView, SignalGet};
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use request_domain::login::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BananaState {
    pub name: String,
    pub wearing: String,
    pub descending: String,
    pub count: u32,
}

impl Default for BananaState {
    fn default() -> Self {
        Self {
            name: "Bananas".to_string(),
            wearing: "pyjamas".to_string(),
            descending: "stairs".to_string(),
            count: 2,
        }
    }
}

#[component]
pub fn test() -> impl IntoView {
    let (banana, _, _) = use_local_storage::<Token, JsonCodec>("token");
    view! { <div>{move || banana.get().token}</div> }
}
