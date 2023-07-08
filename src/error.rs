use axum::{
    response::{IntoResponse, Response},
    Json,
};
use derive_more::{Display, Error};
use http::StatusCode;
use serde_json::json;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Bad request")]
    BadRequest,
    #[display(fmt = "Internal server error")]
    InternalServerError,
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "Invalid input")]
    Validation,
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest | ApiError::Validation => StatusCode::BAD_REQUEST,
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code(), Json(json!({"error": self.to_string()}))).into_response()
    }
}
