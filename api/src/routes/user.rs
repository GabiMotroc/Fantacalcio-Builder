use crate::services::user_service;
use axum::routing::post;
use axum::Router;
use crate::models::app_state::AppState;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(user_service::register))
        .route("/login", post(user_service::login))
}
