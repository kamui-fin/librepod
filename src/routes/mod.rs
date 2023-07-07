use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db,
    error::AppError,
    feed::{delta_update_feed, get_rss_data, update_all_feeds, RssData},
    AppState,
};

#[derive(Deserialize)]
struct AddChannel {
    rss_link: String,
}

async fn get_channel_list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let channels = db::get_channels(&state.pool).await?;
    Ok(Json(channels))
}

async fn add_channel(
    State(mut state): State<AppState>,
    Json(input): Json<AddChannel>,
) -> Result<impl IntoResponse, AppError> {
    let data = get_rss_data(&input.rss_link, &mut state.redis_manager)
        .await
        .ok_or(anyhow!("could not fetch feed"))?;
    let res = db::add_channel(&data.channel, &state.pool).await?;
    // also import missing episodes since you already took the time to fetch RSS
    delta_update_feed(&state.pool, &data).await?;

    Ok(Json(json!({ "ok": res })))
}

async fn delete_channel(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let res = db::delete_channel(id, &state.pool).await?;
    Ok(Json(json!({ "ok": res })))
}

async fn retrieve_feed(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let episodes = db::get_episodes(&state.pool).await?;
    Ok(Json(episodes))
}

async fn refresh_feed(State(mut state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    update_all_feeds(&mut state.redis_manager, &state.pool).await?;
    Ok(Json(json!({"ok": true})))
}

pub fn build_router() -> Router<AppState> {
    let channel_routes = Router::new().route(
        "/",
        get(get_channel_list)
            .post(add_channel)
            .delete(delete_channel),
    );

    let feed_routes = Router::new()
        .route("/", get(retrieve_feed))
        .route("/refresh", put(refresh_feed));

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/channel", channel_routes)
        .nest("/feed", feed_routes)
}
