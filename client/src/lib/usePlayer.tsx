import React from "react"
import { useGlobalAudioPlayer } from "react-use-audio-player"
import { Episode } from "./types"
import { markPlayed } from "./api"
import { createCtx } from "./utils"

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
}

export const [usePlayer, CtxProvider] = createCtx<
    PlayerContextProps | undefined
>()

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [queue, setQueue] = React.useState<Episode[]>([])
    const [stack, setStack] = React.useState<Episode[]>([]) // used for temporary history
    const { load, seek, duration, getPosition } = useGlobalAudioPlayer()

    const startIfNeeded = (episode: Episode) => {
        load(episode.audio_link, {
            autoplay: false,
            html5: true,
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

    const addToFront = (episode: Episode) => {
        startIfNeeded(episode)
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
            }}
        >
            {children}
        </CtxProvider>
    )
}
