use axum::Router;
use axum::routing::post;

use crate::models::app_state::AppState;
use crate::services::player_service;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/search", post(player_service::get_random_players))
}