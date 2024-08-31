use dotenv::dotenv;
use tower_http::cors::CorsLayer;

use crate::models::app_state::AppState;

mod models;
mod startup;
mod services;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_state = AppState {
        db_connection: startup::init_database_pool().await,
    };

    let app = axum::Router::new()
        .nest("/api", routes::get_routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    println!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    axum::serve(listener, app).await?;

    Ok(())
}
