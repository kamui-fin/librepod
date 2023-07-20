use std::time::Duration;

use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use feed_rs::model;
use feed_rs::model::Content;
use feed_rs::model::Image;
use feed_rs::model::Link;
use feed_rs::model::MediaContent;
use feed_rs::model::MediaObject;
use feed_rs::model::Text;
use futures::future;
use futures::future::join_all;
use mime::Mime;
use sqlx::postgres::types::PgInterval;
use sqlx::PgPool;
use sqlx::Row;
use url::Url;
use uuid::Uuid;

use crate::cache::get_response_with_cache;
use crate::cache::CachedHttpResponse;
use crate::models::*;

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
    let recent_date = get_channel_last_published(pool, &data.channel.id).await?;
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

pub async fn delete_channel(id: &str, pool: &PgPool) -> Result<bool> {
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
        DbPodcastChannel,
        r#"
        SELECT channel.* FROM user_subscriptions
        LEFT JOIN channel ON channel.id = channel_id
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    let channels = future::try_join_all(channels.into_iter().map(|c| expand_db_channel(c, pool)))
        .await
        .unwrap();
    Ok(channels)
}

pub async fn add_subscription(user_id: Uuid, channel_id: &str, pool: &PgPool) -> Result<bool> {
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

pub async fn delete_subscription(user_id: Uuid, channel_id: &str, pool: &PgPool) -> Result<bool> {
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

pub async fn get_channels(pool: &PgPool) -> Result<Vec<PodcastChannel>> {
    let channels = sqlx::query_as!(
        DbPodcastChannel,
        r#"
        SELECT * FROM channel
        "#,
    )
    .fetch_all(pool)
    .await?;

    let channels = future::try_join_all(channels.into_iter().map(|c| expand_db_channel(c, pool)))
        .await
        .unwrap();

    Ok(channels)
}

pub async fn get_channel(id: &str, pool: &PgPool) -> Result<Option<PodcastChannel>> {
    let channel = sqlx::query_as!(
        DbPodcastChannel,
        r#"
        SELECT * FROM channel WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(channel) = channel {
        Ok(Some(expand_db_channel(channel, pool).await?))
    } else {
        Ok(None)
    }
}

fn link_from_href(href: String) -> Link {
    Link {
        href,
        rel: None,
        media_type: None,
        href_lang: None,
        title: None,
        length: None,
    }
}

pub async fn expand_db_channel(channel: DbPodcastChannel, pool: &PgPool) -> Result<PodcastChannel> {
    let DbPodcastChannel {
        id,
        rss_link,
        website_link,
        title,
        language,
        description_text_id,
        logo_id,
        icon_id,
    } = channel;
    let description = sqlx::query!(
        "SELECT * FROM text_content WHERE id = $1",
        description_text_id
    )
    .map(|row| Text {
        src: row.src,
        content: row.content,
        content_type: row.content_type.parse().unwrap(),
    })
    .fetch_optional(pool)
    .await?;
    let logo = sqlx::query!("SELECT * FROM image WHERE id = $1", logo_id)
        .map(|row| Image {
            uri: row.uri,
            title: row.title,
            link: row.website_link.map(link_from_href),
            width: row.width.map(|n| n as u32),
            height: row.height.map(|n| n as u32),
            description: row.description,
        })
        .fetch_optional(pool)
        .await?;
    let icon = sqlx::query!("SELECT * FROM image WHERE id = $1", icon_id)
        .map(|row| Image {
            uri: row.uri,
            title: row.title,
            link: row.website_link.map(link_from_href),
            width: row.width.map(|n| n as u32),
            height: row.height.map(|n| n as u32),
            description: row.description,
        })
        .fetch_optional(pool)
        .await?;
    let authors = sqlx::query_as!(
        model::Person,
        r#"
        SELECT name, uri, email FROM person
        LEFT JOIN channel_author ON channel_author.person_id = person.id
        WHERE channel_author.channel_id = $1
        "#,
        id
    )
    .fetch_all(pool)
    .await?;
    let contributors = sqlx::query_as!(
        model::Person,
        r#"
        SELECT name, uri, email FROM person
        LEFT JOIN channel_contributor ON channel_contributor.person_id = person.id
        WHERE channel_contributor.channel_id = $1
        "#,
        id
    )
    .fetch_all(pool)
    .await?;
    let categories = sqlx::query_as!(
        model::Category,
        r#"
        SELECT term, label, null as scheme FROM category
        LEFT JOIN channel_category ON channel_category.category_id = category.id
        WHERE channel_category.channel_id = $1
        "#,
        id
    )
    .fetch_all(pool)
    .await?;

    let num_episodes = sqlx::query!(
        r#"
        SELECT COUNT(*) FROM episode
        WHERE channel_id = $1
    "#,
        id
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or_default() as usize;
    let channel = PodcastChannel {
        id,
        title,
        rss_link,
        website_link,
        language,
        description,
        logo,
        icon,
        authors,
        contributors,
        categories,
        num_episodes,
    };
    Ok(channel)
}

fn pg_interval_to_duration(interval: PgInterval) -> Duration {
    Duration::from_micros(interval.microseconds as u64)
}

pub async fn expand_db_episode(episode: DbPodcastEpisode, pool: &PgPool) -> Result<PodcastEpisode> {
    let DbPodcastEpisode {
        id,
        channel_id,
        title,
        website_link,
        published,
        content_id,
        summary_text_id,
        media_object_id,
    } = episode;
    let media = sqlx::query!("SELECT * FROM media_object WHERE id = $1", media_object_id)
        .map(|row| MediaObject {
            title: None,
            duration: row.duration.clone().map(pg_interval_to_duration),
            content: vec![MediaContent {
                url: row.url.map(|u| Url::parse(&u).unwrap()),
                content_type: row
                    .content_type
                    .map(|c| c.parse().unwrap_or(mime::TEXT_PLAIN_UTF_8)),
                height: row.height.map(|n| n as u32),
                width: row.width.map(|n| n as u32),
                size: row.size.map(|n| n as u64),
                duration: row.duration.map(pg_interval_to_duration),
                rating: None,
            }],

            thumbnails: vec![],
            texts: vec![],
            description: None,
            community: None,
            credits: vec![],
        })
        .fetch_optional(pool)
        .await?;
    let content = sqlx::query!("SELECT * FROM content WHERE id = $1", content_id)
        .map(|row| Content {
            body: row.body,
            content_type: row
                .content_type
                .unwrap_or_default()
                .parse()
                .unwrap_or(mime::TEXT_PLAIN_UTF_8),
            length: row.length.map(|n| n as u64),
            src: row.src.map(link_from_href),
        })
        .fetch_optional(pool)
        .await?;
    let description = sqlx::query!("SELECT * FROM text_content WHERE id = $1", summary_text_id)
        .map(|row| Text {
            src: row.src,
            content: row.content,
            content_type: row.content_type.parse().unwrap(),
        })
        .fetch_optional(pool)
        .await?;
    let authors = sqlx::query_as!(
        model::Person,
        r#"
        SELECT name, uri, email FROM person
        LEFT JOIN channel_author ON channel_author.person_id = person.id
        WHERE channel_author.channel_id = $1
        "#,
        id
    )
    .fetch_all(pool)
    .await?;
    let categories = sqlx::query_as!(
        model::Category,
        r#"
        SELECT term, label, null as scheme FROM category
        LEFT JOIN channel_category ON channel_category.category_id = category.id
        WHERE channel_category.channel_id = $1
        "#,
        id
    )
    .fetch_all(pool)
    .await?;
    let episode = PodcastEpisode {
        id,
        title,
        published,
        website_link,
        summary: description,
        authors,
        categories,
        source_id: channel_id,
        content,
        media: media.unwrap(), // double check if safe
    };
    Ok(episode)
}

pub async fn get_subscription_episodes(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<PodcastEpisode>> {
    let episodes = sqlx::query_as!(
        DbPodcastEpisode,
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
    let episodes = future::try_join_all(episodes.into_iter().map(|c| expand_db_episode(c, pool)))
        .await
        .unwrap();
    Ok(episodes)
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
