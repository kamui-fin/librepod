import Axios from "axios"
import { Episode, Subscription } from "./types"
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
    }
)

export const getSubscriptions = async (): Promise<Subscription[]> => {
    const { data } = await axios.get<Subscription[]>("/channel")
    return data
}

export const getSubscription = async (
    context: LoaderFunctionArgs
): Promise<Subscription> => {
    const id = context.params.name
    const { data } = await axios.get<Subscription>(`/channel/${id}`)
    return data
}

export const addSubscription = async (
    rssLink: string
): Promise<Subscription> => {
    const { data } = await axios.post<Subscription>("/channel", {
        rss_link: rssLink,
    })
    return data
}

export const deleteSubscription = async (
    channelId: string
): Promise<OkResponse> => {
    const { data } = await axios.delete<OkResponse>(`/channel/${channelId}`)
    return data
}

export const getFeed = async (): Promise<Episode[]> => {
    try {
        const { data } = await axios.get<Episode[]>("/feed")
        for (let episode of data) {
            episode.channel = await getSubscription(episode.source_id)
        }
        return data
    } catch {
        return []
    }
}
