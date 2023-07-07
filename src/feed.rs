use crate::cache::{get_response_with_cache, CachedHttpResponse, HttpResponse};
use crate::models::*;
use feed_rs::model::Feed;
use feed_rs::parser;
use reqwest::Client;
use std::time::Duration;

fn get_episode_data(feed: &Feed, rss_link: String) -> Vec<PodcastEpisode> {
    let source = get_channel_data(feed, rss_link);
    if let Some(source) = source {
        feed.entries
            .iter()
            .filter_map(|item| {
                if item.title.is_none() || item.media.is_empty() || item.links.is_empty() {
                    None
                } else {
                    Some(PodcastEpisode {
                        source_id: source.id.clone(),
                        id: item.id.clone(),
                        title: item.title.clone().unwrap().content,
                        published: item.published,
                        authors: item.authors.clone().into_iter().map(Into::into).collect(),
                        content: item.content.clone().into(),
                        website_link: item.links.clone()[0].href.clone(),
                        summary: item.summary.clone().map(Into::into),
                        categories: item
                            .categories
                            .clone()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        media: item.media.get(0).unwrap().clone(),
                    })
                }
            })
            .collect()
    } else {
        vec![]
    }
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
            authors: feed.authors.clone().into_iter().map(Into::into).collect(),
            contributors: feed
                .contributors
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            description: feed.description.clone().map(Into::into),
            categories: feed
                .categories
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            logo: feed.logo.clone().map(Into::into),
            icon: feed.icon.clone().map(Into::into),
            rss_link,
        })
    }
}

pub async fn get_rss_episodes(
    source: &str,
    redis_conn: &mut redis::aio::Connection,
) -> Vec<PodcastEpisode> {
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
        get_episode_data(&feed, source.to_string())
    } else {
        vec![]
    }
}

pub async fn get_rss_source(source: &str) -> Option<PodcastChannel> {
    let client = Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .timeout(Duration::from_secs(60))
    .build().unwrap();
    let response = client
        .get(source)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let feed = parser::parse(&response[..]).unwrap();
    get_channel_data(&feed, source.to_string())
}

pub async fn get_feed(
    sources: Vec<&str>,
    redis_conn: &mut redis::aio::Connection,
) -> Vec<PodcastEpisode> {
    let mut feed = vec![];
    for source in sources {
        feed.extend(get_rss_episodes(source, redis_conn).await);
    }

    feed.sort_by_key(|ep| ep.published);
    feed.reverse();

    feed
}
