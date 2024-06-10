use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::models::error::ApiError::DatabaseError;

pub enum ApiError {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        DatabaseError(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            DatabaseError(e) => {
                if cfg!(debug_assertions) {
                    return
                        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "An unexpected exception has occurred").into_response()
            }
        }
    }
}