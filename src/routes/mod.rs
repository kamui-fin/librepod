mod auth;
mod feed;

use self::auth::*;
use crate::{config::AppContext, models::User};
use async_redis_session::RedisSessionStore;
use axum::{
    routing::{get, put},
    Router,
};
use axum_login::{axum_sessions::SessionLayer, AuthLayer, RequireAuthorizationLayer, SqlxStore};
use feed::*;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

type RequireAuth = RequireAuthorizationLayer<Uuid, User>;

pub fn build_router(
    session_layer: SessionLayer<RedisSessionStore>,
    auth_layer: AuthLayer<SqlxStore<Pool<Postgres>, User>, Uuid, User>,
) -> Router<AppContext> {
    let channel_routes = Router::new().route(
        "/",
        get(get_channel_list)
            .post(add_channel)
            .delete(delete_channel),
    );

    let feed_routes = Router::new()
        .route("/", get(retrieve_feed))
        .route("/refresh", put(refresh_feed));

    let auth_routes = Router::new()
        .route("/me", get(me))
        .route("/logout", put(logout_user))
        .route_layer(RequireAuth::login())
        .route("/register", put(register_user))
        .route("/login", put(login_user));

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/channel", channel_routes)
        .nest("/feed", feed_routes)
        .nest("/auth", auth_routes)
        .layer(auth_layer)
        .layer(session_layer)
}
