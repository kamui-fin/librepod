mod auth;
mod channel;
mod feed;
mod history;
mod models;
mod player;

use self::auth::*;
use self::channel::*;
use self::feed::*;
use self::history::*;
use self::player::*;

use crate::{config::AppContext, core::user::User};

use axum::{
    routing::{get, post, put},
    Router,
};
use axum_login::RequireAuthorizationLayer;

use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

type RequireAuth = RequireAuthorizationLayer<Uuid, User>;

pub fn build_router() -> Router<AppContext> {
    let channel_routes = Router::new()
        .route("/", get(get_subscriptions).post(add_subscription))
        .route("/:id", get(get_subscription).delete(delete_subscription))
        .route_layer(RequireAuth::login());

    let feed_routes = Router::new()
        .route("/", get(retrieve_feed))
        .route("/:id", get(get_episode))
        .route("/refresh", put(refresh_feed))
        .route_layer(RequireAuth::login());

    let auth_routes = Router::new()
        .route("/logout", put(logout_user))
        .route_layer(RequireAuth::login())
        .route("/register", put(register_user))
        .route("/login", put(login_user));

    let history_routes = Router::new()
        .route("/", get(get_history).delete(clear_history))
        .route("/:id", post(add_history));

    let user_routes = Router::new()
        .nest("/history", history_routes)
        .route_layer(RequireAuth::login());

    let player_routes = Router::new()
        .route("/", get(player_ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .route_layer(RequireAuth::login());

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/channel", channel_routes)
        .nest("/feed", feed_routes)
        .nest("/auth", auth_routes)
        .nest("/user", user_routes)
        .nest("/player", player_routes)
}
