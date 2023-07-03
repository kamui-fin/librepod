mod cache;
mod db;
mod feed;

use sqlx::postgres::PgPoolOptions;
// TODO: switch to feed-rs

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // temporary testing
    // assume pubDate exists for now
    /* let now = Instant::now();
    let sources = vec![
        "https://feeds.rebuild.fm/rebuildfm",
        /* "https://getpodcast.xyz/data/ximalaya/12215723.xml",
        "https://getpodcast.xyz/data/ximalaya/29161862.xml",
        "https://getpodcast.xyz/data/163/7.xml",
        "https://getpodcast.xyz/data/ximalaya/13773679.xml",
        "https://tlxxfm.com/feed/podcast", */
    ];
    let eps = get_feed(sources).await;
    println!("Fetched {} episodes.", eps.len());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); */

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/librepod")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    println!("{}", row.0);
    Ok(())
}
