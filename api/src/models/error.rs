use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use hmac::digest::InvalidLength;
use crate::models::error::ApiError::{ConfigurationError, DatabaseError, InvalidCredentials};

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(sqlx::Error),
    ConfigurationError,
    InvalidCredentials,
}

impl From<password_hash::Error> for ApiError
{
    fn from(_: password_hash::Error) -> Self {
        InvalidCredentials
    }
}
impl From<jwt::Error> for ApiError 
{
    fn from(_: jwt::Error) -> Self {
        ConfigurationError
    }
}

impl From<InvalidLength> for ApiError {
    fn from(_: InvalidLength) -> Self {
        ConfigurationError
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        DatabaseError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        println!("{:?}", self);
        match self {
            DatabaseError(e) => {
                if cfg!(debug_assertions) {
                    return
                        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected exception has occurred").into_response()
            }
            ConfigurationError => {
                ("An unexpected error occurred").into_response()
            }
            InvalidCredentials => {
                "Invalid credentials".into_response()
            }
        }
    }
}