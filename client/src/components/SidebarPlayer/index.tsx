import { BiSkipNext, BiSkipPrevious } from "react-icons/bi"
import cx from "classnames"
import { Range, getTrackBackground } from "react-range"
import { useEffect, useRef, useState } from "react"
import { MdPause, MdReplay10, MdForward10 } from "react-icons/md"
import styles from "./style.module.scss"
import { BsFillPlayFill } from "react-icons/bs"
import { usePlayer } from "../../lib/usePlayer"
import { stripHtml } from "string-strip-html"
import { Duration } from "luxon"
import { useGlobalAudioPlayer } from "react-use-audio-player"

const formatLuxon = (seconds: number) => {
    return Duration.fromMillis(seconds * 1000).shiftTo(
        "hours",
        "minutes",
        "seconds",
    )
}

const SidebarPlayer = () => {
    const {
        currentEpisode,
        playNext,
        playPrevious,
        skipTenSeconds,
        replayTenSeconds,
        synchronizeState,
        startFrom,
    } = usePlayer()

    const { getPosition, duration, seek, pause, play } = useGlobalAudioPlayer()

    const getPercentFromPosition = (position: number) =>
        (Math.ceil(formatLuxon(position).as("seconds")) * 100) / duration || 0

    const [posPercent, setPosPercent] = useState(0)
    const [paused, setPaused] = useState(true)
    const [dragging, setDragging] = useState(false)

    const [prevGetPositionValue, setPrevGetPositionValue] = useState(0)
    const frameRef = useRef<number>()

    useEffect(() => {
        setPosPercent(getPercentFromPosition(startFrom))
    }, [startFrom, getPercentFromPosition])

    const pos = Math.round(getPosition())
    useEffect(() => {
        if (
            prevGetPositionValue !== null &&
            prevGetPositionValue !== pos &&
            currentEpisode
        ) {
            synchronizeState({
                episode_id: currentEpisode.id,
                player_time: pos,
            })
        }
        setPrevGetPositionValue(pos)
    }, [pos, prevGetPositionValue, currentEpisode])

    useEffect(() => {
        const animate = () => {
            if (!dragging && !paused) {
                setPosPercent(getPercentFromPosition(getPosition()))
                frameRef.current = requestAnimationFrame(animate)
            }
        }

        frameRef.current = window.requestAnimationFrame(animate)

        return () => {
            if (frameRef.current) {
                cancelAnimationFrame(frameRef.current)
            }
        }
    }, [dragging, paused, pos, prevGetPositionValue])

    const pauseUnpause = () => {
        setPaused(!paused)
        if (!paused) {
            pause()
        } else {
            play()
        }
    }

    if (!currentEpisode) {
        return <></>
    }

    return (
        <div className={styles.player}>
            <img src={currentEpisode.channel_image || ""} />
            <h3>{currentEpisode.title.substring(0, 20) + "..."}</h3>
            <p>
                {stripHtml(currentEpisode.description || "").result.substring(
                    0,
                    20,
                ) + "..."}
            </p>
            <div className={styles.slider}>
                <span>{formatLuxon(getPosition()).toFormat("hh:mm:ss")}</span>
                <Range
                    values={[posPercent]}
                    onChange={(values) => {
                        const [pos] = values

                        // pause()
                        // setPaused(true)
                        setDragging(true)
                        setPosPercent(pos)
                    }}
                    onFinalChange={(values) => {
                        const [pos] = values

                        setPosPercent(pos)
                        seek(Math.round((pos * duration) / 100))
                        setDragging(false)

                        play()
                        setPaused(false)
                    }}
                    renderTrack={({ props, children }) => (
                        <div
                            {...props}
                            className={styles.sliderTrack}
                            style={{
                                ...props.style,
                                background: getTrackBackground({
                                    values: [posPercent],
                                    colors: ["#b8c0cc", "#3c4554"],
                                    min: 0,
                                    max: 100,
                                }),
                            }}
                        >
                            {children}
                        </div>
                    )}
                    renderThumb={({ props }) => (
                        <div {...props} className={styles.sliderThumb} />
                    )}
                />
                <span>
                    {formatLuxon(duration).toFormat("hh:mm:ss") || "--:--"}
                </span>
            </div>
            <div className={styles.controls}>
                <div className={styles.iconOnly} onClick={replayTenSeconds}>
                    <MdReplay10 />
                </div>
                <div
                    className={styles.circleBtn}
                    onClick={() => {
                        playPrevious()
                        setPaused(false)
                    }}
                >
                    <BiSkipPrevious />
                </div>
                <div
                    className={cx(styles.circleBtn, styles.pause)}
                    onClick={pauseUnpause}
                >
                    {paused ? <BsFillPlayFill /> : <MdPause />}
                </div>
                <div
                    className={styles.circleBtn}
                    onClick={() => {
                        playNext()
                        setPaused(false)
                    }}
                >
                    <BiSkipNext />
                </div>
                <div className={styles.iconOnly} onClick={skipTenSeconds}>
                    <MdForward10 />
                </div>
            </div>
        </div>
    )
}

export default SidebarPlayer
