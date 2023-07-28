use std::path::PathBuf;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppContext,
    error::AppError,
    models::{gen_uuid, User},
    services::feed,
};

#[derive(Deserialize)]
pub struct AddChannel {
    rss_link: String,
}

pub async fn get_subscriptions(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let channels = feed::get_subscriptions(&state.pool, user.id).await?;
    Ok(Json(channels))
}

pub async fn get_subscription(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let channel = feed::get_channel(id, &state.pool).await?;
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
) -> Result<impl IntoResponse, AppError> {
    let data = feed::get_rss_data(&input.rss_link, &mut state.redis_manager)
        .await
        .ok_or(anyhow!("could not fetch feed"))?;

    if (feed::get_channel(data.channel.id, &state.pool).await?).is_none() {
        feed::add_channel(&data.channel, &state.pool).await?;
    }

    feed::add_subscription(user.id, data.channel.id, &state.pool).await?;

    // also import missing episodes since you already took the time to fetch RSS
    // side effect that delays result, find alternative
    feed::delta_update_feed(&state.pool, &data).await?;

    Ok(Json(data.channel))
}

pub async fn delete_channel(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let res = feed::delete_subscription(user.id, id, &state.pool).await?;
    Ok(Json(json!({ "ok": res })))
}

pub async fn get_episode(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let episode = feed::get_episode(id, &state.pool).await?;
    Ok(Json(episode))
}

pub async fn retrieve_feed(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let episodes = feed::get_subscription_episodes(user.id, &state.pool).await?;
    Ok(Json(episodes))
}

pub async fn refresh_feed(
    Extension(user): Extension<User>,
    State(mut state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    feed::update_all_feeds(&mut state.redis_manager, &state.pool).await?;
    Ok(Json(json!({"ok": true})))
}

pub async fn get_history(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let episodes = feed::get_history(user.id, &state.pool).await?;
    Ok(Json(episodes))
}

pub async fn clear_history(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let result = feed::clear_history(user.id, &state.pool).await?;
    Ok(Json(json!({ "ok": result })))
}
