use crate::{config::AppContext, core::user::User, error::ApiError, services::history};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn add_history(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    history::mark_played(user.id, id, &state.pool).await?;
    Ok(Json(json!({"ok": true})))
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
    let result = history::clear_history(user.id, &state.pool).await?;
    Ok(Json(json!({ "ok": result })))
}
