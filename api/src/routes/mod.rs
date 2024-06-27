use axum::Router;
use axum::routing::get;
use crate::models::app_state::AppState;

pub mod user;
mod players;

pub fn get_routes() -> Router<AppState>{
    Router::new()
        .route("/hello-world", get(|| async { "Hello World!" }))
        .nest("/user", user::get_routes())
}