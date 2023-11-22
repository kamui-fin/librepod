use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{config::AppContext, core::user::User, error::ApiError, services::feed};

use super::models::PaginationParams;

pub async fn get_episode(
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let episode = feed::get_episode(id, &state.pool).await?;
    Ok(Json(episode))
}

pub async fn retrieve_feed(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, ApiError> {
    let (offset, limit) = (params.offset.unwrap_or(0), params.limit.unwrap_or(15));
    let episodes = feed::get_subscription_episodes(user.id, &state.pool, offset, limit).await?;
    Ok(Json(episodes))
}

pub async fn refresh_feed(
    Extension(_user): Extension<User>,
    State(mut state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    feed::update_all_feeds(&mut state.redis_manager, &state.pool).await?;
    Ok(Json(json!({"ok": true})))
}
