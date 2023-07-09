use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{config::AppContext, error::AppError, models::User, services::feed};

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

pub async fn add_subscription(
    Extension(user): Extension<User>,
    State(mut state): State<AppContext>,
    Json(input): Json<AddChannel>,
) -> Result<impl IntoResponse, AppError> {
    let data = feed::get_rss_data(&input.rss_link, &mut state.redis_manager)
        .await
        .ok_or(anyhow!("could not fetch feed"))?;

    if let None = feed::get_channel(&data.channel.id, &state.pool).await? {
        feed::add_channel(&data.channel, &state.pool).await?;
    }
    // also import missing episodes since you already took the time to fetch RSS
    feed::delta_update_feed(&state.pool, &data).await?;

    let res = feed::add_subscription(user.id, &data.channel.id, &state.pool).await?;

    Ok(Json(json!({ "ok": res })))
}

pub async fn delete_channel(
    Extension(user): Extension<User>,
    Path(id): Path<String>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let res = feed::delete_subscription(user.id, &id, &state.pool).await?;
    Ok(Json(json!({ "ok": res })))
}

pub async fn retrieve_feed(
    Extension(user): Extension<User>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let episodes = feed::get_subscription_episodes(user.id, & state.pool).await?;
    Ok(Json(episodes))
}

pub async fn refresh_feed(
    Extension(user): Extension<User>,
    State(mut state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    feed::update_all_feeds(&mut state.redis_manager, &state.pool).await?;
    Ok(Json(json!({"ok": true})))
}
