use axum::routing::{get, post};
use dotenv::dotenv;
use log::info;
use tower_http::cors::CorsLayer;

use crate::models::app_state::AppState;
use crate::services::user_service;

mod models;
mod startup;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_state = AppState {
        db_connection: startup::init_database_pool().await,
    };

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/user/sighup", post(user_service::signup))
        .route("/login", post(user_service::login))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    axum::serve(listener, app).await?;

    Ok(())
}
