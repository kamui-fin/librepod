use crate::cache::{get_response_with_cache, CachedHttpResponse, HttpResponse};
use crate::{db, models::*};
use feed_rs::model::Feed;
use feed_rs::parser;
use reqwest::Client;
use sqlx::PgPool;
use std::time::Duration;

fn get_episode_data(feed: &Feed, source: &PodcastChannel, rss_link: String) -> Vec<PodcastEpisode> {
    feed.entries
        .iter()
        .filter_map(|item| {
            if item.title.is_none()
                || item.media.is_empty()
                || item.links.is_empty()
                || item.published.is_none()
            {
                None
            } else {
                Some(PodcastEpisode {
                    source_id: source.id.clone(),
                    id: item.id.clone(),
                    title: item.title.clone().unwrap().content,
                    published: item.published.unwrap(),
                    authors: item.authors.clone(),
                    content: item.content.clone(),
                    website_link: item.links.clone()[0].href.clone(),
                    summary: item.summary.clone(),
                    categories: item.categories.clone(),
                    media: item.media.get(0).unwrap().clone(),
                })
            }
        })
        .collect()
}

pub fn get_channel_data(feed: &Feed, rss_link: String) -> Option<PodcastChannel> {
    if feed.title.is_none() || feed.links.is_empty() {
        None
    } else {
        Some(PodcastChannel {
            id: feed.id.clone(),
            title: feed.title.clone().unwrap().content,
            website_link: feed.links.clone()[0].href.clone(),
            language: feed.language.clone(),
            authors: feed.authors.clone(),
            contributors: feed.contributors.clone(),
            description: feed.description.clone(),
            categories: feed.categories.clone(),
            logo: feed.logo.clone(),
            icon: feed.icon.clone(),
            rss_link,
        })
    }
}

pub struct RssData {
    pub channel: PodcastChannel,
    pub episodes: Vec<PodcastEpisode>,
}

pub async fn get_rss_data(
    source: &str,
    redis_conn: &mut redis::aio::ConnectionManager,
) -> Option<RssData> {
    // before request
    let client = Client::builder()
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
        let feed = parser::parse(&http_response.body[..]).unwrap();
        if let Some(channel) = get_channel_data(&feed, source.into()) {
            let mut episodes = get_episode_data(&feed, &channel, source.to_string());
            episodes.sort_by_key(|ep| ep.published);
            return Some(RssData { channel, episodes });
        }
    }
    None
}

pub async fn delta_update_feed(pool: &PgPool, data: &RssData) -> anyhow::Result<()> {
    let recent_date = db::get_channel_last_published(pool, &data.channel.id).await?;
    let update_episodes = if let Some(recent_date) = recent_date {
        let slice = data.episodes.as_slice();
        let low = slice.partition_point(|e| e.published <= recent_date);
        &slice[low..]
    } else {
        &data.episodes[..]
    };

    let tx = pool.begin().await?;
    for episode in update_episodes {
        db::add_episode(episode, pool).await?;
    }
    tx.commit().await?;

    Ok(())
}

pub async fn update_all_feeds(
    redis_conn: &mut redis::aio::ConnectionManager,
    pool: &PgPool,
) -> anyhow::Result<()> {
    let channels = db::get_channels(pool).await?;
    for source in channels.iter().map(|s| s.rss_link.clone()) {
        if let Some(rss_data) = get_rss_data(&source, redis_conn).await {
            delta_update_feed(pool, &rss_data).await?;
        }
    }
    Ok(())
}
