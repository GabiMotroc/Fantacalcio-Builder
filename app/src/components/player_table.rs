use leptos::{component, IntoView};

use request_domain::player::Player;

#[component]
pub fn PlayerTable(players: Vec<Player>) -> impl IntoView {}