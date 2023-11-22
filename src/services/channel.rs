use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::rss::PodcastChannel;

pub async fn add_channel(channel: &PodcastChannel, pool: &PgPool) -> Result<bool> {
    let PodcastChannel {
        id,
        title,
        rss_link,
        website_link,
        author,
        description,
        tags,
        num_episodes: _,
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
