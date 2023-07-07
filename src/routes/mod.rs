use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::{db, error::AppError, AppState};

async fn retrieve_feed(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let episodes = db::get_episodes(&state.pool).await?;
    Ok(Json(episodes))
}

pub fn build_router() -> Router<AppState> {
    let channel_routes = Router::new();

    let feed_routes = Router::new().route("/", get(retrieve_feed));

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/channel", channel_routes)
        .nest("/feed", feed_routes)
}
