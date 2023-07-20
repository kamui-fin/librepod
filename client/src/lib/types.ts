export interface Episode {
    source_id: string
    id: string
    title: string
    published: number
    website_link: string
    content: EpisodeContent
    summary: Text
    authors: any[]
    categories: any[]
    media: Media
}

export interface EpisodeContent {
    body: string | null
    content_type: string
    length: number | null
    src: string | null
}

export interface Media {
    title: string | null
    content: MediaContent[]
    duration: Duration | null
    thumbnails: MediaThumbnail[]
    texts: MediaText[]
    description: Text | null
    community: null
    credits: null
}

export interface MediaText {
    text: Text
    start_time: Duration | null
    end_time: Duration | null
}

export interface MediaThumbnail {
    image: Image
    time: Duration | null
}

export interface MediaContent {
    url: string | null
    content_type: string | null
    height: number | null
    width: number | null
    duration: Duration | null
    size: number | null
    rating: null
}

export interface Duration {
    secs: number
    nanos: number
}

export interface Subscription {
    id: string
    title: string
    rss_link: string
    website_link: string
    language: string | null
    description: Text | null
    logo: Image | null
    icon: Image | null
    authors: Person[]
    contributors: Person[]
    categories: Category[]
    num_episodes: number
}

export interface Person {
    name: string
    uri: string | null
    email: string | null
}

export interface Category {
    term: string | null
    scheme: string | null
    label: string | null
}

export interface Text {
    content_type: string
    src: string | null
    content: string
}

export interface Image {
    uri: string
    title: string | null
    link: Link
    width: number | null
    height: number | null
    description: string | null
}

export interface Link {
    href: string
    rel: string | null
    media_type: string | null
    href_lang: string | null
    title: string | null
    length: number | null
}

export interface ChannelEpisodes {
    channel: Subscription
    episodes: Episode[]
}
