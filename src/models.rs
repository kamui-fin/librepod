use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Content, Image, MediaObject, Person, Text};
use feed_rs::model::{Entry, Feed};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct PodcastChannel {
    pub id: String,
    pub title: String,
    pub rss_link: String,
    pub website_link: String,
    pub language: Option<String>,
    pub description: Option<Text>,
    pub logo: Option<Image>,
    pub icon: Option<Image>,
    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub categories: Vec<Category>,
}

impl PodcastChannel {
    pub fn from_feed(feed: &Feed, rss_link: String) -> Option<Self> {
        if feed.title.is_none() || feed.links.is_empty() {
            None
        } else {
            Some(Self {
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
}

#[derive(Debug, Clone)]
pub struct PodcastEpisode {
    pub source_id: String,
    pub id: String,
    pub title: String,
    pub published: DateTime<Utc>,
    pub website_link: String,
    pub content: Option<Content>,
    pub summary: Option<Text>,
    pub authors: Vec<Person>,
    pub categories: Vec<Category>,
    pub media: MediaObject,
}

impl PodcastEpisode {
    pub fn from_feed_item(item: &Entry, source: &PodcastChannel) -> Option<Self> {
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
    }
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

// Db result structs
// SQLx limitation: just high-level barebones sqlx result, nested field population required
#[derive(Debug, Clone, Serialize)]
pub struct DbPodcastEpisode {
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub website_link: String,
    pub published: DateTime<Utc>,
    pub content_id: Option<i32>,
    pub summary_text_id: Option<i32>,
    pub media_object_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct DbPodcastChannel {
    pub id: String,
    pub rss_link: String,
    pub website_link: String,
    pub title: String,
    pub description_text_id: Option<i32>,
    pub language: Option<String>,
    pub logo_id: Option<i32>,
    pub icon_id: Option<i32>,
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
