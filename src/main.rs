use chrono::{DateTime, TimeZone, Utc};
use rss::{Channel, Enclosure, Item};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
struct Subscription {
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
struct Episode {
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

    source: Subscription,
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

fn get_episode_data(channel: &Channel, rss_link: String) -> Vec<Episode> {
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

            let ep = Episode {
                id,
                title,
                link,
                description,
                content,
                pub_date,
                enclosure,
                source: source.clone(),
                ..Episode::default()
            };

            if let Some(itunes) = item.itunes_ext() {
                let author = itunes.author().map(ToString::to_string);
                let image = itunes.image().map(ToString::to_string);
                let keywords = itunes.keywords().map(ToString::to_string);
                let subtitle = itunes.subtitle().map(ToString::to_string);
                let content = itunes.summary().map(ToString::to_string);

                Some(Episode {
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

fn get_channel_data(channel: &Channel, rss_link: String) -> Subscription {
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

    let sub = Subscription {
        id: hash_subscription_id(channel),
        website_link,
        title,
        description,
        language,
        rss_link,
        image,
        ..Subscription::default()
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

        Subscription {
            author,
            keywords,
            complete,
            subtitle,
            image,
            categories: itunes_categories,
            ..sub
        }
    } else {
        Subscription { categories, ..sub }
    }
}

async fn process_rss(source: &str) -> Vec<Episode> {
    let content = reqwest::get(source).await.unwrap().bytes().await.unwrap();
    let channel = Channel::read_from(&content[..]).unwrap();
    get_episode_data(&channel, source.to_string())
}

async fn get_feed(sources: Vec<&str>) -> Vec<Episode> {
    let mut feed = vec![];
    for source in sources {
        feed.extend(process_rss(source).await);
    }
    feed.sort_by_key(|ep| ep.pub_date);
    feed.reverse();
    feed
}

#[tokio::main]
async fn main() {
    // temporary testing
    // assume pubDate exists for now
    let sources = vec![
        "https://feeds.rebuild.fm/rebuildfm",
        "https://getpodcast.xyz/data/ximalaya/12215723.xml",
        "https://getpodcast.xyz/data/ximalaya/29161862.xml",
        "https://getpodcast.xyz/data/163/7.xml",
        "https://getpodcast.xyz/data/ximalaya/13773679.xml",
        "https://tlxxfm.com/feed/podcast",
    ];
    for ep in get_feed(sources).await {
        println!("{} {:#?}", ep.source.title, ep.pub_date.unwrap_or_default());
    }
}
