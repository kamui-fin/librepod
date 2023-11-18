use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Entry, Feed};
use serde::Serialize;
use sqlx::{postgres::PgRow, Decode, FromRow, Row};
use uuid::Uuid;

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

fn tags_from_categories(categories: Vec<Category>) -> Option<String> {
    let tags = categories
        .clone()
        .iter()
        .map(|c| c.term.clone())
        .collect::<Vec<String>>()
        .join(",");
    (!tags.is_empty()).then(|| tags)
}

pub fn gen_uuid(guid: String) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_URL, guid.as_bytes())
}

impl PodcastChannel {
    pub fn from_feed(feed: &Feed, rss_link: String) -> Option<Self> {
        if feed.title.is_none() || feed.links.is_empty() {
            None
        } else {
            Some(Self {
                id: gen_uuid(feed.id.clone()),
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
                id: gen_uuid(item.id.clone()),
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

// Including channel because usually episode only doesn't make sense without context of channel
#[derive(Serialize)]
pub struct EpisodeWithChannel {
    pub episode: PodcastEpisode,
    pub channel: PodcastChannel,
}

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

#[derive(Clone, Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,
}

impl AuthUser<Uuid> for User {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }
}
