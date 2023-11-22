use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::core::rss::PodcastEpisodeDbResult;

pub async fn get_history(user_id: Uuid, pool: &PgPool) -> Result<Vec<PodcastEpisodeDbResult>> {
    let episodes = sqlx::query_as!(
        PodcastEpisodeDbResult,
        r#"
        SELECT e.*, c.title as channel_title, c.image as channel_image
        FROM user_watch_history as wh
        LEFT JOIN episode AS e ON e.id = wh.episode_id
        LEFT JOIN channel AS c ON c.id = e.channel_id
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
