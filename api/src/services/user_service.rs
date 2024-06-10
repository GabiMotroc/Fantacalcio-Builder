use argon2::{Argon2, password_hash::{
    PasswordHasher, rand_core::OsRng, SaltString,
}};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use log::info;
use password_hash::Error;

use request_domain::login::LoginRequest;

use crate::models::app_state::AppState;
use crate::models::error::ApiError;
use crate::services::db::Db;

pub async fn login(State(state): State<AppState>, Json(login_details): Json<LoginRequest>) -> Response {
    info!("login");
    (StatusCode::CREATED, login_details.email).into_response()
}

pub async fn signup(State(state): State<AppState>, Json(login_details): Json<LoginRequest>) -> Result<Response, ApiError> {
    info!("signup");

    let (hash, salt) = create_password_hash(&login_details.password).unwrap();

    let user_id = Db::create_user(&login_details.email, &hash, &salt, &state.db_connection)
        .await?;

    Ok((StatusCode::CREATED, user_id.to_string()).into_response())
}

fn create_password_hash(password: &str) -> Result<(String, String), Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(
        password.as_bytes(),
        &salt,
    )?;

    Ok((password_hash.to_string(), salt.to_string()))
}
