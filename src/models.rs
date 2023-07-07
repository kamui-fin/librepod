use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Content, Image, MediaObject, Person, Text};

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

pub struct DbPodcastChannel {
    id: String,
    rss_link: String,
    website_link: String,
    title: String,
    description_text_id: Option<i32>,
    language: Option<String>,
    logo_id: Option<i32>,
    icon_id: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct PodcastEpisode {
    pub source_id: String,
    pub id: String,
    pub title: String,
    pub website_link: String,
    pub published: Option<DateTime<Utc>>,
    pub content: Option<Content>,
    pub summary: Option<Text>,
    pub authors: Vec<Person>,
    pub categories: Vec<Category>,
    pub media: MediaObject,
}
