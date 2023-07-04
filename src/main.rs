mod cache;
mod db;
mod feed;

use std::time::Instant;

use feed::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    // temporary testing
    // assume pubDate exists for now
    let now = Instant::now();
    let sources = vec![
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        /* "https://getpodcast.xyz/data/ximalaya/12215723.xml",
        "https://getpodcast.xyz/data/ximalaya/29161862.xml",
        "https://getpodcast.xyz/data/163/7.xml",
        "https://getpodcast.xyz/data/ximalaya/13773679.xml",
        "https://tlxxfm.com/feed/podcast", */
    ];
    let eps = get_feed(sources, &mut con).await;
    // println!("{:#?}", &eps[0..10]);
    println!("Fetched {} episodes.", eps.len());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    /*
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/librepod")
        .await?; */

    Ok(())
}
