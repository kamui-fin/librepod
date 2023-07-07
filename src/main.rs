mod cache;
mod db;
mod error;
mod feed;
mod models;
mod routes;

use crate::routes::build_router;
use anyhow::Result;
use redis::aio::ConnectionManager;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    redis_manager: ConnectionManager,
    pool: PgPool,
}

async fn start_server() -> Result<()> {
    let redis_conn_uri =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string());
    let client = redis::Client::open(redis_conn_uri).unwrap();
    let redis_manager = ConnectionManager::new(client).await.unwrap();

    let pg_conn_uri = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/librepod".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&pg_conn_uri)
        .await
        .expect("can't connect to postgres db");

    let state = AppState {
        redis_manager,
        pool,
    };
    let app = build_router().with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await
}
