use std::time::Duration;

use anyhow::{Context, Result};
use dotenv::dotenv;
use envconfig::Envconfig;
use redis::aio::ConnectionManager;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool};

#[derive(Envconfig, Debug, Clone)]
pub struct Config {
    #[envconfig(from = "API_HOST", default = "0.0.0.0")]
    pub api_host: String,
    #[envconfig(from = "API_PORT", default = "3000")]
    pub api_port: u16,
    #[envconfig(
        from = "DATABASE_URL",
        default = "postgres://postgres:postgres@localhost/librepod"
    )]
    pub db_url: String,
    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1")]
    pub redis_url: String,
    #[envconfig(from = "IMAGE_STORAGE_PATH", default = "/srv/librepod")]
    pub image_storage_path: String,
}

#[derive(Clone)]
pub struct AppContext {
    pub redis_manager: ConnectionManager,
    pub pool: PgPool,
    pub config: Config,
}

impl Config {
    pub fn new() -> Result<Self, envconfig::Error> {
        dotenv().ok();
        Config::init_from_env()
    }
}

pub async fn create_db_pool(pg_conn_uri: &str) -> Result<Pool<sqlx::Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(pg_conn_uri)
        .await
        .context("failed to connect to pg")
}

pub async fn create_redis_manager(redis_conn_uri: &str) -> Result<ConnectionManager> {
    let client = redis::Client::open(redis_conn_uri).unwrap();
    ConnectionManager::new(client)
        .await
        .context("failed to connect to redis")
}

pub fn get_config() -> Config {
    Config::new().expect("Environmental variables need to be set")
}

pub fn get_app_uri() -> String {
    let config = get_config();
    format!("{}:{}", config.api_host, config.api_port)
}

pub async fn init_context() -> AppContext {
    let config = get_config();
    let pool = create_db_pool(&config.db_url)
        .await
        .expect("Failed to create a database pool");
    let redis_manager = create_redis_manager(&config.redis_url)
        .await
        .expect("Failed to connect to redis");
    AppContext {
        pool,
        redis_manager,
        config,
    }
}
