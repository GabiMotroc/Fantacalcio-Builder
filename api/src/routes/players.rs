use axum::routing::{get, post};
use axum::Router;

use crate::models::app_state::AppState;
use crate::services::player_service;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/search", post(player_service::get_players))
        .route("/select", post(player_service::save_selected_players))
        .route("/selected", get(player_service::get_selected_players))
}