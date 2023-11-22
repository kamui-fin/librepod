mod config;
mod core;
mod error;
mod routes;
mod services;

use crate::core::user::User;
use crate::routes::build_router;
use anyhow::{Context, Result};
use async_redis_session::RedisSessionStore;
use axum_login::axum_sessions::SessionLayer;
use axum_login::{AuthLayer, PostgresStore};
use config::{get_app_uri, init_context};

use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use tracing_subscriber;

async fn start_fetch_feed_job() -> Result<()> {
    // generate feed job every 2 hrs
    let sched = JobScheduler::new().await?;
    sched
        .add(Job::new_repeated_async(
            Duration::from_secs(60 * 60 * 2),
            |_, _| {
                Box::pin(async move {
                    let client = reqwest::Client::new();
                    let res = client
                        .put(format!("http://{}/feed/refresh", get_app_uri()))
                        .send()
                        .await;
                    println!("Periodic Sync status: {:#?}", res);
                })
            },
        )?)
        .await?;
    sched
        .start()
        .await
        .context("could not start feed generation job")
}

async fn start_server() -> Result<()> {
    tracing_subscriber::fmt::init();

    let state = init_context().await;
    let app_url = get_app_uri();

    start_fetch_feed_job().await?;

    /* let mut secret = [0; 64];
    rand::thread_rng().fill(&mut secret); */

    let secret = b"please do not hardcode your secret; instead use a
    cryptographically secure value";

    let session_store = RedisSessionStore::new("redis://127.0.0.1")?;
    let session_layer = SessionLayer::new(session_store, secret)
        .with_session_ttl(Some(Duration::from_secs(60 * 60 * 24 * 30)))
        .with_same_site_policy(axum_login::axum_sessions::SameSite::None)
        .with_http_only(true);

    let user_store = PostgresStore::<User>::new(state.pool.clone())
        .with_query("SELECT * FROM account WHERE id = $1");
    let auth_layer = AuthLayer::new(user_store, secret);

    let cors = CorsLayer::very_permissive();

    let app = build_router()
        .with_state(state)
        .layer(auth_layer)
        .layer(session_layer)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    axum::Server::bind(&app_url.parse().unwrap())
        .serve(app)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await
}
