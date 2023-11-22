use anyhow::Result;

use crate::core::rss::get_rss_data;
use crate::core::rss::PodcastEpisode;
use crate::core::rss::PodcastEpisodeDbResult;
use crate::core::rss::RssData;
use sqlx::PgPool;
use uuid::Uuid;

use super::channel::get_channel_last_published;
use super::channel::get_channels;

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

pub async fn get_subscription_episodes(
    user_id: Uuid,
    pool: &PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<PodcastEpisodeDbResult>> {
    let episodes = sqlx::query_as!(
        PodcastEpisodeDbResult,
        r#"
        SELECT e.*, c.title as channel_title, c.image as channel_image
        FROM user_subscriptions AS us
        LEFT JOIN episode AS e ON e.channel_id = us.channel_id
        LEFT JOIN channel AS c ON c.id = e.channel_id
        WHERE user_id = $1
        ORDER BY published DESC
        OFFSET $2
        LIMIT $3
        "#,
        user_id,
        offset,
        limit
    )
    .fetch_all(pool)
    .await?;
    Ok(episodes)
}

// TODO: Add pagination
pub async fn get_channel_episodes(
    channel_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<PodcastEpisodeDbResult>> {
    let episodes = sqlx::query_as!(
        PodcastEpisodeDbResult,
        r#"
        SELECT e.*, c.title as channel_title, c.image as channel_image
        FROM episode AS e
        LEFT JOIN channel AS c ON c.id = e.channel_id
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

pub async fn get_episode(
    episode_id: Uuid,
    pool: &PgPool,
) -> Result<Option<PodcastEpisodeDbResult>> {
    let episode = sqlx::query_as!(
        PodcastEpisodeDbResult,
        r#"
        SELECT e.*, c.title as channel_title, c.image as channel_image
        FROM user_subscriptions AS us
        LEFT JOIN episode AS e ON e.channel_id = us.channel_id
        LEFT JOIN channel AS c ON c.id = e.channel_id
        WHERE e.id = $1
        "#,
        episode_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(episode)
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
