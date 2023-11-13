export interface Subscription {
    id: string
    title: string
    rss_link: string
    website_link: string
    description: string | null
    author: string | null
    tags: string | null
    num_episodes: number | null
    image: string | null
}

export type SubscriptionById = {
    [id: string]: Subscription;
}

export interface Episode {
    id: string
    channel_id: string
    title: string
    published: number
    website_link: string
    content: string | null
    description: string | null
    tags: string | null
    audio_link: string
}

export interface ChannelEpisodes {
    channel: Subscription
    episodes: Episode[]
}

export interface ChannelEpisode {
    channel: Subscription
    episode: Episode
}
