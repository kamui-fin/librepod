import React, { useEffect, useRef } from "react"
import { Duration } from "luxon"
import { useGlobalAudioPlayer } from "react-use-audio-player"

const PlayerContext = React.createContext({})

export interface QueueItem {
    audioUrl: string
    elapsedTime: Duration
}

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [queue, setQueue] = React.useState<QueueItem[]>([])
    const [stack, setStack] = React.useState<QueueItem[]>([]) // used for temporary history
    const [currentEpisode, setCurrentEpisode] =
        React.useState<QueueItem | null>(null) // used for temporary history
    const { load, getPosition, duration, seek, stop, src, paused, error } =
        useGlobalAudioPlayer()
    const frameRef = useRef<number>()

    const addToQueue = (episode: QueueItem) => {
        if (!currentEpisode) {
            setCurrentEpisode(episode)
        }
        setQueue([...queue, episode])
    }

    const addToFront = (episode: QueueItem) => {
        setQueue([episode, ...queue])
        setCurrentEpisode(episode)
    }

    const playNext = () => {
        if (currentEpisode) {
            setStack([...stack, currentEpisode])
            setQueue(queue.slice(1))
            setCurrentEpisode(queue[1] || null)
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
        if (currentEpisode) {
            seek(Math.min(duration, getPosition() + 10))
        }
    }

    const replayTenSeconds = () => {
        if (currentEpisode) {
            seek(Math.max(0, getPosition() - 10))
        }
    }

    // keep state position and global audio context position in sync
    const setCurrentDuration = (elapsedTime: Duration) => {
        if (currentEpisode) {
            const newEpisode = { ...currentEpisode, elapsedTime }
            setCurrentEpisode(newEpisode)
            setQueue([newEpisode, ...queue.slice(1)])
        }
    }
    useEffect(() => {
        const animate = () => {
            setCurrentDuration(Duration.fromMillis(getPosition() * 1000))
            frameRef.current = requestAnimationFrame(animate)
        }

        frameRef.current = window.requestAnimationFrame(animate)

        return () => {
            if (frameRef.current) {
                cancelAnimationFrame(frameRef.current)
            }
        }
    }, [getPosition])

    useEffect(() => {
        if (!currentEpisode) {
            stop()
        } else if (currentEpisode.audioUrl !== src) {
            load(currentEpisode.audioUrl, {
                autoplay: true,
                html5: true,
                onend: playNext,
            })
        }
    }, [currentEpisode])

    return (
        <PlayerContext.Provider
            value={{
                addToQueue,
                addToFront,
                playNext,
                playPrevious,
                skipTenSeconds,
                replayTenSeconds,
                setCurrentDuration,
                currentEpisode,
                queue,
                paused,
                error
            }}
        >
            {children}
        </PlayerContext.Provider>
    )
}

export const usePlayerContext = () => {
    const context = React.useContext(PlayerContext)
    return context
}
