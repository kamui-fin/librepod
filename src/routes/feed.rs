use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{config::AppContext, error::AppError, services::feed};

#[derive(Deserialize)]
pub struct AddChannel {
    rss_link: String,
}

pub async fn get_channel_list(
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let channels = feed::get_channels(&state.pool).await?;
    Ok(Json(channels))
}

pub async fn add_channel(
    State(mut state): State<AppContext>,
    Json(input): Json<AddChannel>,
) -> Result<impl IntoResponse, AppError> {
    let data = feed::get_rss_data(&input.rss_link, &mut state.redis_manager)
        .await
        .ok_or(anyhow!("could not fetch feed"))?;
    let res = feed::add_channel(&data.channel, &state.pool).await?;
    // also import missing episodes since you already took the time to fetch RSS
    feed::delta_update_feed(&state.pool, &data).await?;

    Ok(Json(json!({ "ok": res })))
}

pub async fn delete_channel(
    Path(id): Path<String>,
    State(state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    let res = feed::delete_channel(id, &state.pool).await?;
    Ok(Json(json!({ "ok": res })))
}

pub async fn retrieve_feed(State(state): State<AppContext>) -> Result<impl IntoResponse, AppError> {
    let episodes = feed::get_episodes(&state.pool).await?;
    Ok(Json(episodes))
}

pub async fn refresh_feed(
    State(mut state): State<AppContext>,
) -> Result<impl IntoResponse, AppError> {
    feed::update_all_feeds(&mut state.redis_manager, &state.pool).await?;
    Ok(Json(json!({"ok": true})))
}
