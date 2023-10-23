import { BiSkipNext, BiSkipPrevious } from "react-icons/bi"
import cx from "classnames"
import { Range, getTrackBackground } from "react-range"
import { useEffect, useState } from "react"
import { MdPause, MdReplay10, MdForward10 } from "react-icons/md"
import styles from "./style.module.scss"
import { BsFillPlayFill } from "react-icons/bs"
import { usePlayerContext } from "../../lib/usePlayer"
import { useOutletContext } from "react-router-dom"
import { Duration } from "luxon"
import { stripHtml } from "string-strip-html"

const SidebarPlayer = ({ test }) => {
    const [values, setValues] = useState([0])
    const [paused, setPaused] = useState(true)
    const { episodes, subsById } = test
    const {
        currentEpisode,
        duration,
        addToQueue,
        pause,
        play,
        queue,
        getLuxonTotalDuration,
        playNext,
        playPrevious,
        skipTenSeconds,
        replayTenSeconds,
        seek,
        currentDuration,
        queueFromList,
    } = usePlayerContext()

    useEffect(() => {
        queueFromList(episodes.slice(0, 6))
    }, [])

    useEffect(() => {
        setValues([Math.ceil(currentDuration.as("seconds")) || 0])
    }, [currentDuration])

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
            <img src={subsById[currentEpisode.channel_id].image} />
            <h3>{currentEpisode.title.substring(0, 20) + "..."}</h3>
            <p>
                {stripHtml(currentEpisode.description).result.substring(0, 20) +
                    "..."}
            </p>
            <div className={styles.slider}>
                <span>{currentDuration.toFormat("hh:mm:ss")}</span>
                <Range
                    values={values}
                    min={0}
                    max={duration || 1}
                    step={1}
                    onChange={(values) => {
                        pause()
                        setPaused(true)
                        seek(values[0])
                        setValues(values)
                    }}
                    onFinalChange={(values) => {
                        play()
                        setPaused(false)
                        setValues(values)
                    }}
                    renderTrack={({ props, children }) => (
                        <div
                            {...props}
                            className={styles.sliderTrack}
                            style={{
                                ...props.style,
                                background: getTrackBackground({
                                    values,
                                    colors: ["#b8c0cc", "#3c4554"],
                                    min: 0,
                                    max: duration,
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
                    {getLuxonTotalDuration().toFormat("hh:mm:ss") || "--:--"}
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
