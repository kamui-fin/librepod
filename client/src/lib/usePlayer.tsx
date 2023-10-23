import React, { useEffect, useRef, useState } from "react"
import { Duration } from "luxon"
import { useGlobalAudioPlayer } from "react-use-audio-player"
import { Episode } from "./types"
import { axios } from "./api"

const PlayerContext = React.createContext({})

export const PlayerProvider = ({ children }: { children: React.ReactNode }) => {
    const [queue, setQueue] = React.useState<Episode[]>([])
    const [stack, setStack] = React.useState<Episode[]>([]) // used for temporary history
    const [currentDuration, setCurrentDuration] = React.useState<Duration>(
        Duration.fromMillis(0),
    ) // used for temporary history
    const [currentEpisode, setCurrentEpisode] = React.useState<Episode | null>(
        null,
    ) // used for temporary history
    const {
        load,
        getPosition,
        duration,
        seek,
        stop,
        src,
        paused,
        error,
        pause,
        play,
    } = useGlobalAudioPlayer()
    const frameRef = useRef<number>()

    const queueFromList = (episodes: Episode[]) => {
        if (!currentEpisode) {
            setCurrentEpisode(episodes[0])
        }
        setQueue([...queue, ...episodes])
    }

    const addToQueue = (episode: Episode) => {
        if (!currentEpisode) {
            setCurrentEpisode(episode)
        }
        setQueue([...queue, episode])
    }

    const addToFront = (episode: Episode) => {
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

    const clearQueue = () => {
        setQueue([])
        setCurrentEpisode(null)
    }

    const getLuxonTotalDuration = () => {
        return Duration.fromMillis(duration * 1000).shiftTo(
            "hours",
            "minutes",
            "seconds",
        )
    }

    useEffect(() => {
        const animate = () => {
            setCurrentDuration(
                Duration.fromMillis(getPosition() * 1000).shiftTo(
                    "hours",
                    "minutes",
                    "seconds",
                ),
            )
            frameRef.current = requestAnimationFrame(animate)
        }

        frameRef.current = window.requestAnimationFrame(animate)

        return () => {
            if (frameRef.current) {
                cancelAnimationFrame(frameRef.current)
            }
        }
    }, [getPosition])

    const markPlayed = (episode) => {
        axios.post(`/history/${episode.id}`)
    }

    useEffect(() => {
        if (!currentEpisode) {
            stop()
        } else if (currentEpisode.audio_link !== src) {
            load(currentEpisode.audio_link, {
                autoplay: false,
                html5: true,
                onend: () => {
                    playNext()
                    markPlayed(currentEpisode)
                },
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
                error,
                pause,
                duration,
                play,
                getLuxonTotalDuration,
                getPosition,
                seek,
                currentDuration,
                queueFromList,
                clearQueue,
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
