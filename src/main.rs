use chrono::{DateTime, TimeZone, Utc};
use http::{request, response};
use http_cache_semantics::{AfterResponse, BeforeRequest, CachePolicy, RequestLike, ResponseLike};
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Request, RequestBuilder, Response};
use rss::{Channel, Enclosure, Item};
use std::borrow::Cow;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, time::Instant};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
struct PodcastChannel {
    id: Uuid, // based on link
    rss_link: String,
    website_link: String,
    title: String,
    author: Option<String>,
    description: String,
    subtitle: Option<String>,
    keywords: Option<String>,
    categories: Vec<String>,
    complete: Option<bool>,
    language: Option<String>,
    image: Option<String>,
}

#[derive(Debug, Default)]
struct PodcastEpisode {
    id: Uuid, // uuid v5, based on guid -> link -> title -> throw out  ?
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    content: Option<String>, // itunes summary or actual content
    pub_date: Option<DateTime<Utc>>,
    enclosure: Option<Enclosure>,

    // itunes
    author: Option<String>,
    image: Option<String>,
    keywords: Option<String>,
    subtitle: Option<String>,

    source: PodcastChannel,
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

fn get_episode_data(channel: &Channel, rss_link: String) -> Vec<PodcastEpisode> {
    let source = get_channel_data(channel, rss_link);
    channel
        .items()
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

fn get_channel_data(channel: &Channel, rss_link: String) -> PodcastChannel {
    // collect all subscription metadata
    let website_link = channel.link().to_string();
    let title = channel.title().to_string();
    let description = channel.description().to_string();
    let language = channel.language().map(ToString::to_string);

    let image = channel.image().map(|img| img.url().to_string());
    let mut categories: Vec<String> = channel
        .categories()
        .iter()
        .map(|c| c.name().to_string())
        .collect();
    categories.sort();
    categories.dedup();

    let sub = PodcastChannel {
        id: hash_subscription_id(channel),
        website_link,
        title,
        description,
        language,
        rss_link,
        image,
        ..PodcastChannel::default()
    };

    if let Some(itunes) = channel.itunes_ext() {
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

fn update_request_parts(request: RequestBuilder, parts: request::Parts) -> RequestBuilder {
    request.headers(parts.headers().clone())
}

fn update_response_parts(response: &mut Response, modified_parts: response::Parts) {
    let headers = response.headers_mut();
    for (key, value) in modified_parts.headers() {
        headers.insert(key.clone(), value.clone());
    }
}

async fn process_rss(
    source: &str,
    cache: &mut HashMap<String, (CachePolicy, Response)>,
) -> Vec<PodcastEpisode> {
    // before request
    let client = Client::new();
    let mut request_builder = client.get(source);
    let mut orig_request = request_builder.try_clone().unwrap().build().unwrap();

    let final_bytes;

    // try to pull from cache if possible
    if let Some((policy, cached_response)) = cache.get_mut(&source.to_string()) {
        final_bytes = match policy.before_request(&orig_request, SystemTime::now()) {
            BeforeRequest::Fresh(cached_response_parts) => {
                println!("HIT CACHE!");
                update_response_parts(cached_response, cached_response_parts);
                cached_response.bytes().await.unwrap()
            }
            BeforeRequest::Stale { request, matches } => {
                // update parts
                request_builder = update_request_parts(request_builder, request);
                orig_request = request_builder.try_clone().unwrap().build().unwrap();

                let mut response = request_builder.send().await.unwrap();
                match policy.after_response(&orig_request, &response, SystemTime::now()) {
                    AfterResponse::NotModified(policy, parts) => {
                        // 304
                        // use cached body, update headers from parts
                        println!("HIT CACHE!");
                        update_response_parts(cached_response, parts);
                        cached_response.bytes().await.unwrap()
                    }
                    AfterResponse::Modified(policy, parts) => {
                        // 200
                        // still have to update headers
                        update_response_parts(&mut response, parts);
                        // update body in cache
                        cache.insert(source.to_string(), (policy, response));
                        response.bytes().await.unwrap()
                    }
                }
            }
        }
    } else {
        let mut response = request_builder.send().await.unwrap();
        let cache_policy = CachePolicy::new(&orig_request, &response);
        if cache_policy.is_storable() {
            println!("STORING IN CACHE");
            cache.insert(source.to_string(), (cache_policy, response));
        }
        final_bytes = response.bytes().await.unwrap();
    }

    let channel = Channel::read_from(&final_bytes[..]).unwrap();
    get_episode_data(&channel, source.to_string())
}

async fn get_feed(sources: Vec<&str>) -> Vec<PodcastEpisode> {
    let mut cache: HashMap<String, (CachePolicy, Response)> = HashMap::new();
    let mut feed = vec![];
    for source in sources {
        feed.extend(process_rss(source, &mut cache).await);
    }
    feed.sort_by_key(|ep| ep.pub_date);
    feed.reverse();
    feed
}

#[tokio::main]
async fn main() {
    // temporary testing
    // assume pubDate exists for now
    let now = Instant::now();
    let sources = vec![
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        "https://feeds.rebuild.fm/rebuildfm",
        /* "https://getpodcast.xyz/data/ximalaya/12215723.xml",
        "https://getpodcast.xyz/data/ximalaya/29161862.xml",
        "https://getpodcast.xyz/data/163/7.xml",
        "https://getpodcast.xyz/data/ximalaya/13773679.xml",
        "https://tlxxfm.com/feed/podcast", */
    ];
    for ep in get_feed(sources).await {
        println!("{} {:#?}", ep.source.title, ep.pub_date.unwrap_or_default());
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
