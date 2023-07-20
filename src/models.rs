use axum_login::secrecy::SecretVec;
use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use feed_rs::model::{
    Category, Content, Image, Link, MediaCommunity, MediaContent, MediaCredit, MediaObject,
    MediaRating, MediaText, MediaThumbnail, Person, Text,
};
use feed_rs::model::{Entry, Feed};
use mime::Mime;
use serde::Serialize;
use sqlx::FromRow;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, Default)]
pub struct PodcastChannel {
    pub id: String,
    pub title: String,
    pub rss_link: String,
    pub website_link: String,
    pub language: Option<String>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_text")]
    pub description: Option<Text>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_image")]
    pub logo: Option<Image>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_image")]
    pub icon: Option<Image>,
    #[serde(serialize_with = "custom_serde::serialize_vec_person")]
    pub authors: Vec<Person>,
    #[serde(serialize_with = "custom_serde::serialize_vec_person")]
    pub contributors: Vec<Person>,
    #[serde(serialize_with = "custom_serde::serialize_vec_category")]
    pub categories: Vec<Category>,

    pub num_episodes: usize,
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
                num_episodes: feed.entries.len(),
            })
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PodcastEpisode {
    pub source_id: String,
    pub id: String,
    pub title: String,
    #[serde(with = "chrono::serde::ts_microseconds")]
    pub published: DateTime<Utc>,
    pub website_link: String,
    #[serde(default, serialize_with = "custom_serde::serialize_option_content")]
    pub content: Option<Content>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_text")]
    pub summary: Option<Text>,
    #[serde(serialize_with = "custom_serde::serialize_vec_person")]
    pub authors: Vec<Person>,
    #[serde(serialize_with = "custom_serde::serialize_vec_category")]
    pub categories: Vec<Category>,
    #[serde(with = "MediaObjectDef")]
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

// Re-defining feed-rs structs due to lack of serde

#[derive(Serialize)]
#[serde(remote = "Category")]
pub struct CategoryDef {
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
}

#[derive(Serialize)]
#[serde(remote = "Content")]
pub struct ContentDef {
    pub body: Option<String>,
    #[serde(with = "mime_serde_shim")]
    pub content_type: Mime,
    pub length: Option<u64>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_link")]
    pub src: Option<Link>,
}

#[derive(Serialize)]
#[serde(remote = "Image")]
pub struct ImageDef {
    pub uri: String,
    pub title: Option<String>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_link")]
    pub link: Option<Link>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(remote = "Link")]
pub struct LinkDef {
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub href_lang: Option<String>,
    pub title: Option<String>,
    pub length: Option<u64>,
}

#[derive(Serialize)]
#[serde(remote = "MediaObject")]
pub struct MediaObjectDef {
    #[serde(default, serialize_with = "custom_serde::serialize_option_text")]
    pub title: Option<Text>,
    #[serde(default, serialize_with = "custom_serde::serialize_vec_media_content")]
    pub content: Vec<MediaContent>,
    pub duration: Option<Duration>,
    #[serde(
        default,
        serialize_with = "custom_serde::serialize_vec_media_thumbnail"
    )]
    pub thumbnails: Vec<MediaThumbnail>,
    #[serde(default, serialize_with = "custom_serde::serialize_vec_media_text")]
    pub texts: Vec<MediaText>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_text")]
    pub description: Option<Text>,
    #[serde(
        default,
        serialize_with = "custom_serde::serialize_option_media_community"
    )]
    pub community: Option<MediaCommunity>,
    #[serde(default, serialize_with = "custom_serde::serialize_vec_media_credit")]
    pub credits: Vec<MediaCredit>,
}

#[derive(Serialize)]
#[serde(remote = "MediaCommunity")]
pub struct MediaCommunityDef {
    pub stars_avg: Option<f64>,
    pub stars_count: Option<u64>,
    pub stars_min: Option<u64>,
    pub stars_max: Option<u64>,

    pub stats_views: Option<u64>,
    pub stats_favorites: Option<u64>,
}

#[derive(Serialize)]
#[serde(remote = "MediaContent")]
pub struct MediaContentDef {
    pub url: Option<Url>,
    #[serde(default, serialize_with = "custom_serde::serialize_option_mime")]
    pub content_type: Option<Mime>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub duration: Option<Duration>,
    pub size: Option<u64>,
    #[serde(
        default,
        serialize_with = "custom_serde::serialize_option_media_rating"
    )]
    pub rating: Option<MediaRating>,
}

#[derive(Serialize)]
#[serde(remote = "MediaCredit")]
pub struct MediaCreditDef {
    pub entity: String,
}

#[derive(Serialize)]
#[serde(remote = "MediaRating")]
pub struct MediaRatingDef {
    pub urn: String,
    pub value: String,
}

#[derive(Serialize)]
#[serde(remote = "MediaText")]
pub struct MediaTextDef {
    #[serde(with = "TextDef")]
    pub text: Text,
    pub start_time: Option<Duration>,
    pub end_time: Option<Duration>,
}

#[derive(Serialize)]
#[serde(remote = "MediaThumbnail")]
pub struct MediaThumbnailDef {
    #[serde(with = "ImageDef")]
    pub image: Image,
    pub time: Option<Duration>,
}

#[derive(Serialize)]
#[serde(remote = "Person")]
pub struct PersonDef {
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize)]
#[serde(remote = "Text")]
pub struct TextDef {
    #[serde(with = "mime_serde_shim")]
    pub content_type: Mime,
    pub src: Option<String>,
    pub content: String,
}

// addressing serde limitations #723
mod custom_serde {
    use super::*;
    use serde::{ser::SerializeSeq, Serialize, Serializer};

    pub fn serialize_option_mime<S>(value: &Option<Mime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "mime_serde_shim")] &'a Mime);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_option_text<S>(value: &Option<Text>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "TextDef")] &'a Text);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_option_link<S>(value: &Option<Link>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "LinkDef")] &'a Link);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_option_content<S>(
        value: &Option<Content>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "ContentDef")] &'a Content);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_option_image<S>(
        value: &Option<Image>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "ImageDef")] &'a Image);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_option_media_rating<S>(
        value: &Option<MediaRating>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "MediaRatingDef")] &'a MediaRating);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_option_media_community<S>(
        value: &Option<MediaCommunity>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "MediaCommunityDef")] &'a MediaCommunity);
        value.as_ref().map(Helper).serialize(serializer)
    }

    pub fn serialize_vec_person<S>(value: &Vec<Person>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "PersonDef")] &'a Person);

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value.iter().map(Helper) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn serialize_vec_category<S>(
        value: &Vec<Category>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "CategoryDef")] &'a Category);

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value.iter().map(Helper) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn serialize_vec_media_thumbnail<S>(
        value: &Vec<MediaThumbnail>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "MediaThumbnailDef")] &'a MediaThumbnail);

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value.iter().map(Helper) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn serialize_vec_media_content<S>(
        value: &Vec<MediaContent>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "MediaContentDef")] &'a MediaContent);

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value.iter().map(Helper) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn serialize_vec_media_text<S>(
        value: &Vec<MediaText>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "MediaTextDef")] &'a MediaText);

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value.iter().map(Helper) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }

    pub fn serialize_vec_media_credit<S>(
        value: &Vec<MediaCredit>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "MediaCreditDef")] &'a MediaCredit);

        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for item in value.iter().map(Helper) {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}
