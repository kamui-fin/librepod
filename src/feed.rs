use crate::cache::{get_response_with_cache, CachedHttpResponse, HttpResponse};
use chrono::{DateTime, Utc};
use feed_rs::model::{Category, Content, Feed, Image, Link, MediaRating, Person, Text};
use feed_rs::parser;
use http_cache_semantics::CachePolicy;
use reqwest::Client;
use rss::{Channel, Item};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct PodcastChannel {
    pub id: String,
    pub title: Option<Text>,
    pub rss_link: String,
    pub links: Vec<Link>,
    pub authors: Vec<Person>, // atom only
    pub contributors: Vec<Person>,
    pub description: Option<Text>,
    pub categories: Vec<Category>,
    pub language: Option<String>,
    pub image: Option<Image>,
    pub icon: Option<Image>,
}

#[derive(Debug, Default)]
pub struct PodcastEpisode {
    pub id: String,
    pub title: Option<Text>,
    pub published: Option<DateTime<Utc>>,
    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub content: Option<Content>,
    pub links: Vec<Link>,
    pub summary: Option<Text>,
    pub categories: Vec<Category>,
    pub enclosure: Option<Enclosure>,
    pub image: Option<String>,
    pub source: PodcastChannel,
}

fn parse_rfc2822_timestamp(timestamp: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let parsed_datetime = DateTime::parse_from_rfc2822(timestamp)?;
    let utc_datetime = parsed_datetime.with_timezone(&Utc);
    Ok(utc_datetime)
}

fn hash_subscription_id(channel: &Channel) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_URL, channel.link.as_bytes())
}

fn hash_episode_id(item: &Item) -> Option<Uuid> {
    let identifier = item
        .guid()
        .map(|g| g.value())
        .or(item.link())
        .or(item.title())?;
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, identifier.as_bytes());
    Some(uuid)
}

fn get_episode_data(feed: &Feed, rss_link: String) -> Vec<PodcastEpisode> {
    let source = get_channel_data(feed, rss_link);
    feed.items()
        .iter()
        .filter_map(|item| {
            let title = item.title().map(ToString::to_string);
            let link = item.link().map(ToString::to_string);
            let enclosure = item.enclosure().cloned();
            let pub_date = item
                .pub_date()
                .map(|date| parse_rfc2822_timestamp(date).unwrap());
            let description = item.description().map(ToString::to_string);
            let content = item.content().map(ToString::to_string);
            let id = hash_episode_id(item)?;

            let ep = PodcastEpisode {
                id,
                title,
                link,
                description,
                content,
                pub_date,
                enclosure,
                source: source.clone(),
                ..PodcastEpisode::default()
            };

            if let Some(itunes) = item.itunes_ext() {
                let author = itunes.author().map(ToString::to_string);
                let image = itunes.image().map(ToString::to_string);
                let keywords = itunes.keywords().map(ToString::to_string);
                let subtitle = itunes.subtitle().map(ToString::to_string);
                let content = itunes.summary().map(ToString::to_string);

                Some(PodcastEpisode {
                    author,
                    image,
                    keywords,
                    subtitle,
                    content,
                    ..ep
                })
            } else {
                Some(ep)
            }
        })
        .collect()
}

fn get_channel_data(feed: &Feed, rss_link: String) -> PodcastChannel {
    // collect all subscription metadata
    let id = feed.id;
    let website_link = feed.links;
    let title = feed.title;
    let description = feed.description;
    let language = feed.language;
    let image = feed.logo.map(|img| img.uri);

    let mut categories: Vec<String> = feed.categories.iter().map(|c| c.term).collect();
    categories.sort();
    categories.dedup();

    let sub = PodcastChannel {
        id,
        website_link,
        title,
        description,
        language,
        rss_link,
        image,
        ..PodcastChannel::default()
    };

    if let Some(itunes) = feed.itunes_ext() {
        let image = itunes.image().map(ToString::to_string);
        let author = itunes.author().map(ToString::to_string);
        let keywords = itunes.keywords().map(ToString::to_string);
        let subtitle = itunes.subtitle().map(ToString::to_string);
        let complete = itunes.complete().map(|c| c == "Yes");

        // only add itunes categories if useful
        let mut itunes_categories: Vec<String> = itunes
            .categories()
            .iter()
            .map(|c| c.text().to_string())
            .collect();
        itunes_categories.sort();
        itunes_categories.dedup();
        if itunes_categories.len() < categories.len() {
            itunes_categories = categories;
        }

        PodcastChannel {
            author,
            keywords,
            complete,
            subtitle,
            image,
            categories: itunes_categories,
            ..sub
        }
    } else {
        PodcastChannel { categories, ..sub }
    }
}

async fn get_feed(sources: Vec<&str>) -> Vec<PodcastEpisode> {
    let mut cache: HashMap<String, (CachePolicy, HttpResponse)> = HashMap::new();
    let mut feed = vec![];
    for source in sources {
        feed.extend(get_rss_episodes(source, &mut cache).await);
    }

    feed.sort_by_key(|ep| ep.pub_date);
    feed.reverse();

    feed
}

pub async fn get_rss_episodes(
    source: &str,
    cache: &mut HashMap<String, (CachePolicy, HttpResponse)>,
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
    let cached_response = get_response_with_cache(request, cache, source).await;
    // only fetch feeds not cached
    if let CachedHttpResponse::Miss(http_response) = cached_response {
        let feed = parser::parse(&http_response.body[..]).unwrap();
        get_episode_data(&feed, source.to_string())
    } else {
        vec![]
    }
}
