use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::{header, StatusCode};
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;

pub struct ApiError {
    pub msg: String,
    pub status_code: StatusCode,
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        let err: anyhow::Error = err.into();
        Self {
            msg: format!("Something went wrong: {err}"),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ApiError {
    pub fn new(msg: &str, status_code: StatusCode) -> Self {
        Self {
            msg: msg.into(),
            status_code,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            [(header::CONTENT_TYPE, "applications/json")],
            Json(json!({ "error": self.msg })),
        )
            .into_response()
    }
}
