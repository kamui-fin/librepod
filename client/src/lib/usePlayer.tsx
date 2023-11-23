import React, { useEffect } from "react"
import { useGlobalAudioPlayer } from "react-use-audio-player"
import { Episode } from "./types"
import { getEpisodeById, markPlayed } from "./api"
import { createCtx } from "./utils"
import useWebSocket from "react-use-websocket"

interface PlayerContextProps {
    addToQueue: (episode: Episode) => void
    addToFront: (episode: Episode) => void
    playNext: () => void
    playPrevious: () => void
    skipTenSeconds: () => void
    replayTenSeconds: () => void
    currentEpisode: Episode | null
    queue: Episode[]
    queueFromList: (episodes: Episode[]) => void
    clearQueue: () => void
    synchronizeState: (playerState: PlayerState) => void
    startFrom: number
}

export const [usePlayer, CtxProvider] = createCtx<
    PlayerContextProps | undefined
>()

interface PlayerState {
    episode_id: string
    player_time: number // seconds
}

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [queue, setQueue] = React.useState<Episode[]>([])
    const [stack, setStack] = React.useState<Episode[]>([]) // used for temporary history
    const [startFrom, setStartFrom] = React.useState(0)
    const { load, seek, duration, getPosition } = useGlobalAudioPlayer()

    const initializePlayer = async (
        episodeId?: string,
        seekSeconds?: number,
    ) => {
        if (episodeId && seekSeconds) {
            const episode = await getEpisodeById(episodeId)
            addToFront(episode, seekSeconds)
            setStartFrom(seekSeconds)
        }
    }

    const {
        sendMessage,
        sendJsonMessage,
        lastMessage,
        lastJsonMessage,
        readyState,
    } = useWebSocket(`ws://localhost:3000/player`, {
        onOpen: () => {
            console.log("Player state synchronization socket opened")
            sendMessage("get_state")
        },
    })

    useEffect(() => {
        if (lastJsonMessage) {
            const { episode_id, player_time } = lastJsonMessage as PlayerState
            initializePlayer(episode_id, player_time)
                .then()
                .catch(console.error)
        }
    }, [lastJsonMessage])

    const synchronizeState = (playerState: PlayerState) => {
        console.log(playerState)
        sendJsonMessage(playerState)
    }

    const startIfNeeded = (episode: Episode, playerTime = 0) => {
        load(episode.audio_link, {
            autoplay: false,
            html5: true,
            onload: () => {
                seek(playerTime)
            },
            onend: () => {
                playNext()
                markPlayed(episode).then().catch(console.error)
            },
        })
    }

    const queueFromList = (episodes: Episode[]) => {
        startIfNeeded(episodes[0])
        setQueue([...queue, ...episodes])
    }

    const addToFront = (episode: Episode, seekSeconds = 0) => {
        startIfNeeded(episode, seekSeconds)
        setQueue([episode, ...queue])
    }

    const addToQueue = (episode: Episode) => {
        setQueue([...queue, episode])
    }

    const playNext = () => {
        if (queue.length) {
            setStack([...stack, queue[0]])
            setQueue(queue.slice(1))
        }
    }

    const playPrevious = () => {
        const previous = stack[stack.length - 1]
        if (previous) {
            setStack(stack.slice(0, stack.length - 1))
            addToFront(previous)
        }
    }

    const skipTenSeconds = () => {
        if (queue.length) {
            seek(Math.min(duration, getPosition() + 10))
        }
    }

    const replayTenSeconds = () => {
        if (queue.length) {
            seek(Math.max(0, getPosition() - 10))
        }
    }

    const clearQueue = () => {
        setQueue(queue.slice(0, 1))
    }

    return (
        <CtxProvider
            value={{
                addToQueue,
                addToFront,
                playNext,
                playPrevious,
                skipTenSeconds,
                replayTenSeconds,
                queueFromList,
                clearQueue,
                queue,
                currentEpisode: queue[0],
                synchronizeState,
                startFrom,
            }}
        >
            {children}
        </CtxProvider>
    )
}
