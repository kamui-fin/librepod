use crate::cache::{get_response_with_cache, CachedHttpResponse, HttpResponse};
use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Content, Feed, Image, MediaObject, Person, Text};
use feed_rs::parser;
use futures::prelude::*;
use http_cache_semantics::CachePolicy;
use redis::{AsyncCommands, Connection};
use reqwest::Client;
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone)]
pub struct PodcastChannel {
    pub id: String,
    pub title: Text,
    pub rss_link: String,
    pub website_link: Option<String>,

    pub description: Option<Text>,
    pub language: Option<String>,
    pub logo: Option<Image>,
    pub icon: Option<Image>,

    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub categories: Vec<Category>,
}

#[derive(Debug)]
pub struct PodcastEpisode {
    pub source_id: String,

    pub id: String,
    pub title: Text,
    pub published: Option<DateTime<Utc>>,
    pub content: Option<Content>,
    pub website_link: Option<String>,
    pub summary: Option<Text>,

    pub authors: Vec<Person>,
    pub categories: Vec<Category>,
    pub media: MediaObject,
}

fn get_episode_data(feed: &Feed, rss_link: String) -> Vec<PodcastEpisode> {
    let source = get_channel_data(feed, rss_link);
    if let Some(source) = source {
        feed.entries
            .iter()
            .filter_map(|item| {
                if item.title.is_none() || item.media.is_empty() {
                    None
                } else {
                    Some(PodcastEpisode {
                        source_id: source.id.clone(),
                        id: item.id.clone(),
                        title: item.title.clone().unwrap(),
                        published: item.published,
                        authors: item.authors.clone(),
                        content: item.content.clone(),
                        website_link: item.links.clone().get(0).map(|link| link.clone().href),
                        summary: item.summary.clone(),
                        categories: item.categories.clone(),
                        media: item.media.get(0).unwrap().clone(),
                    })
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn get_channel_data(feed: &Feed, rss_link: String) -> Option<PodcastChannel> {
    if feed.title.is_none() {
        None
    } else {
        Some(PodcastChannel {
            id: feed.id.clone(),
            title: feed.title.clone().unwrap(),
            website_link: feed.links.clone().get(0).map(|link| link.clone().href),
            authors: feed.authors.clone(),
            contributors: feed.contributors.clone(),
            description: feed.description.clone(),
            categories: feed.categories.clone(),
            language: feed.language.clone(),
            logo: feed.logo.clone(),
            icon: feed.icon.clone(),
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

pub async fn get_feed(sources: Vec<&str>, redis_conn: &mut redis::aio::Connection) -> Vec<PodcastEpisode> {
    // replace with redis
    let mut feed = vec![];
    for source in sources {
        feed.extend(get_rss_episodes(source, redis_conn).await);
    }

    feed.sort_by_key(|ep| ep.published);
    feed.reverse();

    feed
}
