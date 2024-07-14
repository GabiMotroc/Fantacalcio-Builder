use std::collections::BTreeMap;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{Header, VerifyWithKey};
use sha2::Sha256;

use crate::models::error::ApiError;
use crate::models::user::User;

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("X-Auth-Token")
            .and_then(|header| header.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized due to missing header"))?;

        get_user_from_token(auth_header)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized due to bad token"))
    }
}

pub fn get_user_from_token(token: &str) -> Result<User, ApiError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
    let token: jwt::Token<Header, BTreeMap<String, i32>, _> = token.verify_with_key(&key)?;
    let claims = token.claims();
    Ok(User {
        id: claims["Id"],
        email: "".to_string(),
        password: "".to_string(),
        salt: "".to_string(),
        created_at: Default::default(),
        last_login: None,
    })
}
