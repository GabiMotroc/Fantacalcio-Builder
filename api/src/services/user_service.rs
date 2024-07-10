use std::collections::BTreeMap;

use argon2::{Argon2, password_hash::{
    PasswordHasher, rand_core::OsRng, SaltString,
}};
use axum::extract::State;
use axum::Json;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use password_hash::{Error, PasswordHash, Salt};
use sha2::Sha256;

use request_domain::login::{LoginRequest, Token};

use crate::models::app_state::AppState;
use crate::models::error::ApiError;
use crate::models::user::User;
use crate::services::db::Db;

pub async fn login(State(state): State<AppState>, Json(login_details): Json<LoginRequest>) -> Result<Json<Token>, ApiError> {
    println!("login");

    let user = Db::get_user(&login_details.email, &state.db_connection).await?;

    verify_password_hash(&user, &login_details.password)?;

    let token = create_token(user.id)?;

    Ok(
        Json::from(Token {
            token
        })
    )
}

pub async fn register(State(state): State<AppState>, Json(login_details): Json<LoginRequest>) -> Result<Json<Token>, ApiError> {
    println!("signup");

    let (hash, salt) = create_password_hash(&login_details.password).unwrap();

    let user_id = Db::create_user(&login_details.email, &hash, &salt, &state.db_connection)
        .await?;

    let token = create_token(user_id)?;

    Ok(
        Json::from(Token {
            token
        })
    )
}

pub fn create_token(user_id: i32) -> Result<String, ApiError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;

    let mut claims = BTreeMap::new();
    claims.insert("Id", user_id);
    let token_str = claims.sign_with_key(&key)?;
    Ok(token_str)
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

fn verify_password_hash(user: &User, input_password: &str) -> Result<(), ApiError> {
    let salt = Salt::from_b64(&user.salt).unwrap();

    let argon2 = Argon2::default();
    let hash = PasswordHash::generate(argon2, input_password, salt);
    let hash_string = hash.unwrap().to_string();
    let result = hash_string == user.password;

    println!("{} {}", &hash_string, user.password);

    match result {
        true => { Ok(()) }
        false => { Err(ApiError::InvalidCredentials) }
    }
}
