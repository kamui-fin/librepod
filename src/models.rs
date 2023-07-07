use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Content, Image, MediaObject, Person, Text};
use serde::Serialize;

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
