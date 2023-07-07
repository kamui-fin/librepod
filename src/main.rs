mod cache;
mod db;
mod feed;
mod models;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/librepod")
        .await?;

    Ok(())
}
