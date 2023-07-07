use std::time::Duration;

use anyhow::{anyhow, Result};
use chrono::DateTime;
use chrono::Utc;
use futures::future::join_all;
use sqlx::postgres::types::PgInterval;
use sqlx::PgPool;
use sqlx::Row;

use crate::models::*;

pub async fn delete_channel(id: String, pool: &PgPool) -> Result<bool> {
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

pub async fn add_channel(channel: &PodcastChannel, pool: &PgPool) -> Result<bool> {
    let description_id = if let Some(description) = &channel.description {
        sqlx::query!(
            r#"
        INSERT INTO text_content(content, content_type, src)
        VALUES ($1, $2, $3)
        RETURNING id
    "#,
            description.content,
            description.content_type.to_string(),
            description.src,
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.id)
    } else {
        None
    };

    let icon_id = if let Some(icon) = &channel.icon {
        sqlx::query!(
            r#"
        INSERT INTO image(uri, title, website_link, width, height, description)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
    "#,
            icon.uri,
            icon.title,
            icon.link.clone().map(|l| l.href),
            icon.width.map(|w| w as i32),
            icon.height.map(|h| h as i32),
            icon.description
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.id)
    } else {
        None
    };

    let logo_id = if let Some(logo) = &channel.logo {
        sqlx::query!(
            r#"
        INSERT INTO image(uri, title, website_link, width, height, description)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
    "#,
            logo.uri,
            logo.title,
            logo.link.clone().map(|l| l.href),
            logo.width.map(|w| w as i32),
            logo.height.map(|h| h as i32),
            logo.description
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.id)
    } else {
        None
    };

    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO channel(id, title, rss_link, website_link, language, logo_id, icon_id, description_text_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#,
        channel.id,
        channel.title,
        channel.rss_link,
        channel.website_link,
        channel.language,
        logo_id,
        icon_id,
        description_id,
    )
    .execute(pool)
    .await?
    .rows_affected();

    let contributor_ids = channel
        .contributors
        .iter()
        .map(|contributor| {
            sqlx::query!(
                r#"
            INSERT INTO person(name, uri, email)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
                contributor.name,
                contributor.uri,
                contributor.email
            )
            .fetch_one(pool)
        })
        .collect::<Vec<_>>();

    let author_ids = channel
        .authors
        .iter()
        .map(|author| {
            sqlx::query!(
                r#"
            INSERT INTO person(name, uri, email)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
                author.name,
                author.uri,
                author.email
            )
            .fetch_one(pool)
        })
        .collect::<Vec<_>>();

    let category_ids = channel
        .categories
        .iter()
        .map(|category| {
            sqlx::query!(
                r#"
            INSERT INTO category(term, label)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#,
                category.term,
                category.label
            )
            .fetch_one(pool)
        })
        .collect::<Vec<_>>();

    for row in join_all(category_ids).await.into_iter().flatten() {
        sqlx::query!(
            r#"
                    INSERT INTO channel_category(channel_id, category_id)
                    VALUES ($1, $2)
                "#,
            channel.id,
            row.id
        )
        .execute(pool)
        .await?;
    }

    for row in join_all(author_ids).await.into_iter().flatten() {
        sqlx::query!(
            r#"
                    INSERT INTO channel_author(channel_id, person_id)
                    VALUES ($1, $2)
                "#,
            channel.id,
            row.id
        )
        .execute(pool)
        .await?;
    }

    for row in join_all(contributor_ids).await.into_iter().flatten() {
        sqlx::query!(
            r#"
                    INSERT INTO channel_contributor(channel_id, person_id)
                    VALUES ($1, $2)
                "#,
            channel.id,
            row.id
        )
        .execute(pool)
        .await?;
    }

    Ok(rows_affected > 0)
}

pub async fn get_channels(pool: &PgPool) -> Result<Vec<DbPodcastChannel>> {
    let channels = sqlx::query_as!(
        DbPodcastChannel,
        r#"
        SELECT * FROM channel
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(channels)
}

pub async fn get_channel(id: String, pool: &PgPool) -> Result<Option<DbPodcastChannel>> {
    let channel = sqlx::query_as!(
        DbPodcastChannel,
        r#"
        SELECT * FROM channel WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(channel)
}

pub async fn get_episodes(pool: &PgPool) -> Result<Vec<DbPodcastEpisode>> {
    let episodes = sqlx::query_as!(
        DbPodcastEpisode,
        r#"
        SELECT * FROM episode
        ORDER BY published DESC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(episodes)
}

pub async fn get_episode(id: String, pool: &PgPool) -> Result<Option<DbPodcastEpisode>> {
    let episode = sqlx::query_as!(
        DbPodcastEpisode,
        r#"
        SELECT * FROM episode WHERE id = $1
        ORDER BY published DESC
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(episode)
}

pub async fn add_episode(episode: &PodcastEpisode, pool: &PgPool) -> Result<bool> {
    let content_id = if let Some(content) = &episode.content {
        sqlx::query!(
            r#"
        INSERT INTO content(body, content_type, length, src)
        VALUES ($1, $2, $3, $4)
        RETURNING id
    "#,
            content.body,
            content.content_type.to_string(),
            content.length.map(|l| l as i64),
            content.src.clone().map(|s| s.href),
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.id)
    } else {
        None
    };

    let summary_text_id = if let Some(summary) = &episode.summary {
        sqlx::query!(
            r#"
        INSERT INTO text_content(content, content_type, src)
        VALUES ($1, $2, $3)
        RETURNING id
    "#,
            summary.content,
            summary.content_type.to_string(),
            summary.src,
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.id)
    } else {
        None
    };

    let media_object_id = if let Some(media_content) = episode.media.content.get(0) {
        sqlx::query!(
            r#"
                    INSERT INTO media_object(url, duration, content_type, height, width, size)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING id
                "#,
            media_content.url.clone().map(|u| u.to_string()),
            episode
                .media
                .duration
                .map(|dur| PgInterval::try_from(dur))
                .and_then(Result::ok),
            media_content.content_type.clone().map(|t| t.to_string()),
            media_content.height.map(|n| n as i32),
            media_content.width.map(|n| n as i32),
            media_content.size.map(|n| n as i64),
        )
        .fetch_one(pool)
        .await
        .ok()
        .map(|row| row.id)
    } else {
        None
    };

    let rows_affected = sqlx::query!(
        r#"
        INSERT INTO episode(id, channel_id, title, published, content_id, website_link, summary_text_id, media_object_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    "#,
        episode.id,
        episode.source_id,
        episode.title,
        episode.published,
        content_id,
        episode.website_link,
        summary_text_id,
        media_object_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    let author_ids = episode
        .authors
        .iter()
        .map(|author| {
            sqlx::query!(
                r#"
            INSERT INTO person(name, uri, email)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
                author.name,
                author.uri,
                author.email
            )
            .fetch_one(pool)
        })
        .collect::<Vec<_>>();

    let category_ids = episode
        .categories
        .iter()
        .map(|category| {
            sqlx::query!(
                r#"
            INSERT INTO category(term, label)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            RETURNING id
        "#,
                category.term,
                category.label
            )
            .fetch_one(pool)
        })
        .collect::<Vec<_>>();

    for row in join_all(category_ids).await.into_iter().flatten() {
        sqlx::query!(
            r#"
                    INSERT INTO episode_category(episode_id, category_id)
                    VALUES ($1, $2)
                "#,
            episode.id,
            row.id
        )
        .execute(pool)
        .await?;
    }

    for row in join_all(author_ids).await.into_iter().flatten() {
        sqlx::query!(
            r#"
                    INSERT INTO episode_author(episode_id, person_id)
                    VALUES ($1, $2)
                "#,
            episode.id,
            row.id
        )
        .execute(pool)
        .await?;
    }

    Ok(rows_affected > 0)
}

pub async fn get_channel_last_published(
    pool: &PgPool,
    channel_id: &str,
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
