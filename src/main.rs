mod cache;
mod config;
mod db;
mod error;
mod feed;
mod models;
mod routes;

use crate::routes::build_router;
use anyhow::{Context, Result};
use config::{get_app_uri, init_context};
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

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
    let state = init_context().await;
    let app_url = get_app_uri();
    let app = build_router().with_state(state).into_make_service();

    start_fetch_feed_job().await?;

    axum::Server::bind(&app_url.parse().unwrap())
        .serve(app)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    start_server().await
}
