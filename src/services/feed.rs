use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use reqwest::Client;
use sqlx::postgres::types::PgInterval;
use sqlx::PgPool;
use uuid::Uuid;

use crate::cache::get_response_with_cache;
use crate::cache::CachedHttpResponse;
use crate::models::*;

pub async fn download_image(url: &str, filename: &str, directory: &str) -> Result<PathBuf> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(anyhow!("Unable to download image"));
    }
    let image_data = response.bytes().await?;
    let filepath = Path::new(directory).join(filename);
    fs::write(filepath.clone(), image_data)?;
    Ok(filepath)
}

pub async fn get_rss_data(
    source: &str,
    redis_conn: &mut redis::aio::ConnectionManager,
) -> Option<RssData> {
    // before request
    let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .timeout(Duration::from_secs(60))
    .build().unwrap();
    let request = client.get(source);
    let cached_response = get_response_with_cache(request, redis_conn, source).await;
    // only fetch feeds not cached
    if let CachedHttpResponse::Miss(http_response) = cached_response {
        let feed = feed_rs::parser::parse(&http_response.body[..]).unwrap();
        RssData::from_feed(&feed, source.to_string())
    } else {
        None
    }
}

pub async fn delta_update_feed(pool: &PgPool, data: &RssData) -> anyhow::Result<()> {
    let recent_date = get_channel_last_published(pool, data.channel.id).await?;
    let update_episodes = if let Some(recent_date) = recent_date {
        let slice = data.episodes.as_slice();
        let low = slice.partition_point(|e| e.published <= recent_date);
        &slice[low..]
    } else {
        &data.episodes[..]
    };

    let tx = pool.begin().await?;
    for episode in update_episodes {
        add_episode(episode, pool).await?;
    }
    tx.commit().await?;

    Ok(())
}

pub async fn update_all_feeds(
    redis_conn: &mut redis::aio::ConnectionManager,
    pool: &PgPool,
) -> anyhow::Result<()> {
    let channels = get_channels(pool).await?;
    for source in channels.iter().map(|s| s.rss_link.clone()) {
        if let Some(rss_data) = get_rss_data(&source, redis_conn).await {
            delta_update_feed(pool, &rss_data).await?;
        }
    }
    Ok(())
}

pub async fn delete_channel(id: Uuid, pool: &PgPool) -> Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM channel
        WHERE id = $1
    "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn get_subscriptions(pool: &PgPool, user_id: Uuid) -> Result<Vec<PodcastChannel>> {
    let channels = sqlx::query_as!(
       PodcastChannel,
        r#"
        SELECT channel.*, COALESCE((SELECT COUNT(episode.id) FROM episode WHERE episode.channel_id = channel.id), 0) as num_episodes FROM user_subscriptions
        LEFT JOIN channel ON channel.id = channel_id
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(channels)
}

pub async fn add_subscription(user_id: Uuid, channel_id: Uuid, pool: &PgPool) -> Result<bool> {
    let rows_affected = sqlx::query!(
        "INSERT INTO user_subscriptions VALUES ($1, $2)",
        user_id,
        channel_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn delete_subscription(user_id: Uuid, channel_id: Uuid, pool: &PgPool) -> Result<bool> {
    let rows_affected = sqlx::query!(
        "DELETE FROM user_subscriptions WHERE user_id = $1 AND channel_id = $2",
        user_id,
        channel_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn add_channel(channel: &PodcastChannel, pool: &PgPool) -> Result<bool> {
    let PodcastChannel {
        id,
        title,
        rss_link,
        website_link,
        author,
        description,
        tags,
        num_episodes,
        image,
    } = channel;
    let rows_affected = sqlx::query(
        r#"
        INSERT INTO channel(id, title, rss_link, website_link, author, description, tags, image)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#,
    )
    .bind(id)
    .bind(title)
    .bind(rss_link)
    .bind(website_link)
    .bind(author)
    .bind(description)
    .bind(tags)
    .bind(image)
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn get_channels(pool: &PgPool) -> Result<Vec<PodcastChannel>> {
    let channels = sqlx::query_as!(
        PodcastChannel,
        r#"
        SELECT *, COALESCE((SELECT COUNT(episode.id) FROM episode WHERE episode.channel_id = id), 0) as num_episodes FROM channel
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(channels)
}

pub async fn get_channel(id: Uuid, pool: &PgPool) -> Result<Option<PodcastChannel>> {
    let channel = sqlx::query_as!(
        PodcastChannel,
        r#"
        SELECT *, COALESCE((SELECT COUNT(episode.id) FROM episode WHERE episode.channel_id = $1), 0) as num_episodes FROM channel WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(channel)
}

fn pg_interval_to_duration(interval: PgInterval) -> Duration {
    Duration::from_micros(interval.microseconds as u64)
}

pub async fn get_channel_episodes(channel_id: Uuid, pool: &PgPool) -> Result<Vec<PodcastEpisode>> {
    let episodes = sqlx::query_as!(
        PodcastEpisode,
        r#"
        SELECT * FROM episode
        WHERE channel_id = $1
        ORDER BY published DESC
        LIMIT 20
        "#,
        channel_id
    )
    .fetch_all(pool)
    .await?;
    Ok(episodes)
}

pub async fn get_episode(episode_id: Uuid, pool: &PgPool) -> Result<Option<PodcastEpisode>> {
    let episode = sqlx::query_as!(
        PodcastEpisode,
        r#"
        SELECT * FROM episode
        WHERE id = $1
        "#,
        episode_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(episode)
}

pub async fn get_subscription_episodes(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<PodcastEpisode>> {
    let episodes = sqlx::query_as!(
        PodcastEpisode,
        r#"
        SELECT episode.* FROM user_subscriptions
        LEFT JOIN episode ON episode.channel_id = user_subscriptions.channel_id
        WHERE user_id = $1
        ORDER BY published DESC
        LIMIT 20
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(episodes)
}

pub async fn add_episode(episode: &PodcastEpisode, pool: &PgPool) -> Result<bool> {
    let PodcastEpisode {
        channel_id,
        id,
        title,
        website_link,
        published,
        description,
        content,
        tags,
        audio_link,
    } = episode;
    let rows_affected = sqlx::query(
        r#"
        INSERT INTO episode(id, channel_id, website_link, published, title, audio_link, description, content, tags)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    "#,
    )
    .bind(id)
    .bind(channel_id)
    .bind(website_link)
    .bind(published)
    .bind(title)
    .bind(audio_link)
    .bind(description)
    .bind(content)
    .bind(tags)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(rows_affected > 0)
}

pub async fn get_channel_last_published(
    pool: &PgPool,
    channel_id: Uuid,
) -> Result<Option<DateTime<Utc>>> {
    let last_published = sqlx::query!(
        r#"
        SELECT MAX(published) FROM episode
        WHERE channel_id = $1
        GROUP BY channel_id
    "#,
        channel_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = last_published {
        Ok(record.max)
    } else {
        Ok(None)
    }
}

pub async fn get_history(user_id: Uuid, pool: &PgPool) -> Result<Vec<PodcastEpisode>> {
    let episodes = sqlx::query_as!(
        PodcastEpisode,
        r#"
        SELECT episode.* FROM user_watch_history
        LEFT JOIN episode ON episode.id = user_watch_history.episode_id
        WHERE user_id = $1
        ORDER BY published DESC
        LIMIT 20
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(episodes)
}

pub async fn clear_history(user_id: Uuid, pool: &PgPool) -> Result<bool> {
    let result = sqlx::query_as!(
        PodcastEpisode,
        r#"
        DELETE FROM user_watch_history
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn mark_played(user_id: Uuid, episode_id: Uuid, pool: &PgPool) -> Result<bool> {
    let result = sqlx::query_as!(
        PodcastEpisode,
        r#"
        INSERT INTO user_watch_history
        VALUES($1, $2)
        "#,
        user_id,
        episode_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
