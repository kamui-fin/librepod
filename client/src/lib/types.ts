export interface Channel {
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

export type ChannelById = {
    [id: string]: Channel;
}

export interface Episode {
    id: string
    channel: Channel
    title: string
    published: number
    website_link: string
    content: string | null
    description: string | null
    tags: string | null
    audio_link: string
}

export interface ChannelEpisodes {
    channel: Channel
    episodes: Episode[]
}

export interface ChannelEpisode {
    channel: Channel
    episode: Episode
}
