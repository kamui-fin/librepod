mod auth;
mod feed;

use self::auth::*;
use crate::{config::AppContext, models::User};
use async_redis_session::RedisSessionStore;
use axum::{
    routing::{delete, get, put},
    Router,
};
use axum_login::{axum_sessions::SessionLayer, AuthLayer, RequireAuthorizationLayer, SqlxStore};
use feed::*;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

type RequireAuth = RequireAuthorizationLayer<Uuid, User>;

pub fn build_router() -> Router<AppContext> {
    let channel_routes = Router::new()
        .route("/", get(get_subscriptions).post(add_subscription))
        .route("/:id", get(get_subscription).delete(delete_channel))
        .route_layer(RequireAuth::login());

    let feed_routes = Router::new()
        .route("/", get(retrieve_feed))
        .route("/refresh", put(refresh_feed))
        .route_layer(RequireAuth::login());

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
}
