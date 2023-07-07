mod cache;
mod db;
mod feed;
mod models;

use std::time::Instant;

use feed::*;
use sqlx::{postgres::PgPoolOptions, Postgres, Row};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/librepod")
        .await?;

    let result = db::get_channel("600259e547cf0cfd8e04c1144fd196bd".into(), &pool).await;
    if let Ok(result) = result {
        println!("{:#?}", result);
    } else {
        println!("{:#?}", result.err());
    }

    Ok(())
}
