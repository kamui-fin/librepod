use crate::{config::AppContext, core::user::User, error::ApiError, services::history};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use http::StatusCode;
use uuid::Uuid;

pub async fn add_history(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let res = history::mark_played(user.id, id, &state.pool).await?;
    if !res {
        return Err(ApiError::new("episode not found", StatusCode::NOT_FOUND));
    }
    Ok(StatusCode::CREATED)
}

pub async fn get_history(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let episodes = history::get_history(user.id, &state.pool).await?;
    Ok(Json(episodes))
}

pub async fn clear_history(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let res = history::clear_history(user.id, &state.pool).await?;
    if !res {
        return Ok(StatusCode::NO_CONTENT);
    }
    Ok(StatusCode::OK)
}
