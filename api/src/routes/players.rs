use axum::Router;
use crate::models::app_state::AppState;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/search/:name", pla)
}