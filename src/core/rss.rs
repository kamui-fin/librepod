use std::time::Duration;

use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Entry, Feed};
use serde::Serialize;
use uuid::Uuid;

use super::cache::{get_response_with_cache, CachedHttpResponse};

// Data fetched straight from RSS link
pub struct RssData {
    pub channel: PodcastChannel,
    pub episodes: Vec<PodcastEpisode>,
}

impl RssData {
    pub fn from_feed(feed: &Feed, rss_link: String) -> Option<Self> {
        let channel = PodcastChannel::from_feed(feed, rss_link);
        if let Some(channel) = channel {
            let mut episodes = feed
                .entries
                .iter()
                .filter_map(|item| PodcastEpisode::from_feed_item(&item, &channel))
                .collect::<Vec<PodcastEpisode>>();
            episodes.sort_by_key(|ep| ep.published);
            Some(Self { channel, episodes })
        } else {
            None
        }
    }
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

#[derive(Serialize, Debug, Clone, Default)]
pub struct PodcastChannel {
    pub id: Uuid,
    pub title: String,
    pub rss_link: String,
    pub website_link: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub num_episodes: Option<i64>,
    pub image: Option<String>,
}

impl PodcastChannel {
    pub fn from_feed(feed: &Feed, rss_link: String) -> Option<Self> {
        if feed.title.is_none() || feed.links.is_empty() {
            None
        } else {
            Some(Self {
                id: gen_uuid_from_existing_id(feed.id.clone()),
                title: feed.title.clone().unwrap().content,
                website_link: feed.links.clone()[0].href.clone(),
                author: feed.authors.get(0).map(|p| p.name.clone()),
                description: feed.description.clone().map(|t| t.content),
                tags: tags_from_categories(feed.categories.clone()),
                rss_link,
                num_episodes: Some(feed.entries.len() as i64),
                image: feed.logo.clone().map(|l| l.uri),
            })
        }
    }
}

// DB Episode table entity
#[derive(Serialize, Debug, Clone)]
pub struct PodcastEpisode {
    pub channel_id: Uuid,
    pub id: Uuid,
    pub title: String,
    pub website_link: String,
    #[serde(with = "chrono::serde::ts_microseconds")]
    pub published: DateTime<Utc>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>,
    pub audio_link: String,
}

// Sqlx doesn't support nesting well, and neither does rust support inheritance, so have to manually duplicate fields
// This is the struct returned for API calls, providing additional channel context data
// Internally we don't need this
#[derive(Serialize, Debug, Clone)]
pub struct PodcastEpisodeDbResult {
    // base PodcastEpisode
    pub channel_id: Uuid,
    pub id: Uuid,
    pub title: String,
    pub website_link: String,
    #[serde(with = "chrono::serde::ts_microseconds")]
    pub published: DateTime<Utc>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>,
    pub audio_link: String,

    // channel additions
    pub channel_title: String,
    pub channel_image: Option<String>,
}

impl PodcastEpisode {
    pub fn from_feed_item(item: &Entry, source: &PodcastChannel) -> Option<Self> {
        let media = item
            .media
            .get(0)
            .map(|m| {
                m.content
                    .get(0)
                    .map(|m| m.url.clone().map(|u| u.to_string()))
            })
            .flatten()
            .flatten();
        if item.title.is_none()
            || item.media.is_empty()
            || item.links.is_empty()
            || item.published.is_none()
            || media.is_none()
        {
            None
        } else {
            Some(PodcastEpisode {
                channel_id: source.id.clone(),
                id: gen_uuid_from_existing_id(item.id.clone()),
                title: item.title.clone().unwrap().content,
                website_link: item.links.clone()[0].href.clone(),
                published: item.published.unwrap(),
                content: item.content.clone().map(|t| t.body.unwrap_or_default()),
                description: item.summary.clone().map(|t| t.content),
                tags: tags_from_categories(item.categories.clone()),
                audio_link: media.unwrap(),
            })
        }
    }
}

// Struct builder-related helpers

fn tags_from_categories(categories: Vec<Category>) -> Option<String> {
    let tags = categories
        .clone()
        .iter()
        .map(|c| c.term.clone())
        .collect::<Vec<String>>()
        .join(",");
    (!tags.is_empty()).then(|| tags)
}

fn gen_uuid_from_existing_id(guid: String) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_URL, guid.as_bytes())
}
