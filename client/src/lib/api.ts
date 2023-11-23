import Axios, { AxiosError } from "axios"
import { ChannelEpisodes, Episode, Channel } from "./types"
import { LoginBody } from "@/pages/login"
import { RegisterBody } from "@/pages/register"
import { User } from "./useAuth"

export interface OkResponse {
    ok: boolean
}

export const axios = Axios.create({
    baseURL: import.meta.env.VITE_API_URL,
    withCredentials: true,
})

axios.interceptors.response.use(
    (response) => response,
    (error: AxiosError) => {
        if (error.response?.status === 401) {
            localStorage.removeItem("user")
        }
        return Promise.reject(error)
    },
)

export const getSubscriptions = async (): Promise<Channel[]> => {
    const { data } = await axios.get<Channel[]>("/channel")
    return data
}

export const getSubscriptionById = async (
    id: string,
): Promise<ChannelEpisodes> => {
    const { data } = await axios.get<ChannelEpisodes>(`/channel/${id}`)
    return data
}

export const addSubscription = async (rssLink: string): Promise<Channel> => {
    const { data } = await axios.post<Channel>("/channel", {
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

export interface PaginationParams {
    offset: number
    limit: number
}

export const getFeed = async (
    { offset, limit }: PaginationParams = {
        offset: 0,
        limit: 15,
    },
): Promise<Episode[]> => {
    const { data } = await axios.get<Episode[]>("/feed", {
        params: { offset, limit },
    })
    return data
}

export const getEpisodeById = async (id: string): Promise<Episode> => {
    const { data } = await axios.get<Episode>(`/feed/${id}`)
    return data
}

export const getHistory = async (): Promise<Episode[]> => {
    const { data } = await axios.get<Episode[]>("/user/history")
    return data
}

export const clearHistory = async (): Promise<OkResponse> => {
    const { data } = await axios.delete<OkResponse>("/user/history")
    return data
}

export const markPlayed = async (episode: Episode) => {
    await axios.post(`/user/history/${episode.id}`)
}

// Authentication calls

export const logout = async () => {
    await axios.put("/auth/logout")
}

export const loginUser = async (values: LoginBody) => {
    const { data }: { data: User } = await axios.put("/auth/login", values)
    return data
}

export const registerUser = async (values: RegisterBody) => {
    const { data }: { data: User } = await axios.put("/auth/register", values)
    return data
}
