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
    [id: string]: Channel
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
    channel_title: string
    channel_image: string | null
}

export interface ChannelEpisodes {
    channel: Channel
    episodes: Episode[]
}
