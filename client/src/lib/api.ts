import Axios from "axios"
import { ChannelEpisode, ChannelEpisodes, Episode, Subscription } from "./types"
import { LoaderFunctionArgs } from "react-router-dom"

export interface OkResponse {
    ok: boolean
}

export const axios = Axios.create({
    baseURL: import.meta.env.VITE_API_URL,
    withCredentials: true,
})

axios.interceptors.response.use(
    function (response) {
        return response
    },
    function (error) {
        if (error.response.status === 401) {
            localStorage.removeItem("user")
        }
        return Promise.reject(error)
    },
)

export const getSubscriptions = async (): Promise<Subscription[]> => {
    const { data } = await axios.get<Subscription[]>("/channel")
    return data
}

export const getSubscription = async (
    context: LoaderFunctionArgs,
): Promise<ChannelEpisodes> => {
    const id = context.params.name
    try {
        const { data } = await axios.get<ChannelEpisodes>(`/channel/${id}`)
        return data
    } catch (e) {
        console.log(e)
        return {}
    }
}

export const addSubscription = async (
    rssLink: string,
): Promise<Subscription> => {
    const { data } = await axios.post<Subscription>("/channel", {
        rss_link: rssLink,
    })
    return data
}

export const deleteSubscription = async (
    channelId: string,
): Promise<OkResponse> => {
    const { data } = await axios.delete<OkResponse>(`/channel/${channelId}`)
    return data
}

export const getFeed = async (): Promise<Episode[]> => {
    try {
        const { data } = await axios.get<Episode[]>("/feed")
        return data
    } catch (e) {
        return []
    }
}

export const getEpisode = async (
    context: LoaderFunctionArgs,
): Promise<ChannelEpisode> => {
    const id = context.params.name
    try {
        const { data } = await axios.get<Episode>(`/feed/${id}`)
        return {
            episode: data,
            channel: (
                await axios.get<ChannelEpisodes>(`/channel/${data.channel_id}`)
            ).data,
        }
    } catch (e) {
        return {}
    }
}

export const getHistory = async (): Promise<Episode[]> => {
    try {
        const { data } = await axios.get<Episode[]>("/user/history")
        return data
    } catch (e) {
        return []
    }
}

export const feedLoader = async () => {
    try {
        const history = await getHistory()
        const episodes = await getFeed()
        const subs = await getSubscriptions()
        const subsById = {}
        for (let sub of subs) {
            subsById[sub.id] = sub
        }
        return {
            episodes,
            subsById,
            history,
        }
    } catch (e) {
        console.log(e)
        return {}
    }
}

export const clearHistory = async (): Promise<OkResponse> => {
    try {
        const { data } = await axios.delete<OkResponse>("/user/history")
        return data
    } catch (e) {
        return { ok: false }
    }
}
