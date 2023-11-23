use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppContext,
    core::{rss::get_rss_data, user::User},
    error::ApiError,
    services::channel,
    services::feed,
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddChannel {
    rss_link: String,
}

pub async fn get_subscriptions(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let channels = channel::get_subscriptions(&state.pool, user.id).await?;
    Ok(Json(channels))
}

pub async fn get_subscription(
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let channel = channel::get_channel(id, &state.pool).await?;
    if channel.is_none() {
        return Err(ApiError::new("channel not found", StatusCode::NOT_FOUND));
    }
    let episodes = feed::get_channel_episodes(id, &state.pool).await?;
    Ok(Json(json!({
        "channel": channel,
        "episodes": episodes
    })))
}

pub async fn add_subscription(
    Extension(user): Extension<User>,
    State(mut state): State<AppContext>,
    Json(input): Json<AddChannel>,
) -> Result<impl IntoResponse, ApiError> {
    let data = get_rss_data(&input.rss_link, &mut state.redis_manager)
        .await
        .ok_or(anyhow!("could not fetch feed"))?;

    if (channel::get_channel(data.channel.id, &state.pool).await?).is_none() {
        channel::add_channel(&data.channel, &state.pool).await?;
    }

    channel::add_subscription(user.id, data.channel.id, &state.pool).await?;

    // also import missing episodes since you already took the time to fetch RSS
    // side effect that delays result, find alternative
    feed::delta_update_feed(&state.pool, &data).await?;

    Ok(Json(data.channel))
}

pub async fn delete_subscription(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, ApiError> {
    let res = channel::delete_subscription(user.id, id, &state.pool).await?;
    if !res {
        return Err(ApiError::new(
            "subscription not found",
            StatusCode::NOT_FOUND,
        ));
    }
    Ok(StatusCode::OK)
}
