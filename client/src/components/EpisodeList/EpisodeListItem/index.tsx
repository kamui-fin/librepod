import { Link } from "react-router-dom"
import styles from "./style.module.scss"
import { stripHtml } from "string-strip-html"
import { AiFillPlayCircle } from "react-icons/ai"
import { ChannelEpisode, Episode } from "../../../lib/types"
import { getHumanDate } from "../../../lib/utils"
import { MdPlaylistAdd } from "react-icons/md"
import { usePlayer } from "../../../lib/usePlayer"

export interface EpisodeOnly {
    episode: Episode
}

interface Props {
    item: EpisodeOnly | ChannelEpisode,
}

const EpisodeListItem = ({ item }: Props) => {
    const { episode } = item;
    const { addToQueue, addToFront } = usePlayer()
    return (
        <div className={styles.listItem}>
            {!("channel" in item) ? (
                <div className={styles.meta}>
                    <span className={styles.channel}>
                        {getHumanDate(episode.published)}
                    </span>
                    <Link to={`/episode/${episode.id}`}>
                        <h3 className={styles.title}>{episode.title}</h3>
                    </Link>
                    <p className={styles.desc}>
                        {stripHtml(episode.description || '').result.substring(0, 150) +
                            "..."}
                    </p>
                </div>
            ) : (
                <div className={styles.withChannel}>
                    <div className={styles.image}>
                        <img src={item.channel.image || ''} />
                    </div>
                    <div className={styles.meta}>
                        <Link to={`/episode/${episode.id}`}>
                            <h3 className={styles.title}>{episode.title}</h3>
                        </Link>
                        <p className={styles.desc}>
                            {stripHtml(episode?.description || '').result.substring(
                                0,
                                130,
                            ) + "..."}
                        </p>
                        <Link to={`/subscriptions/channel/${item.channel.id}`}>
                            <span className={styles.channel}>
                                {item.channel?.title || ''}
                            </span>
                        </Link>
                    </div>
                </div>
            )}
            <div className={styles.play}>
                <button
                    className={styles.playButton}
                    onClick={() => addToFront(episode)}
                >
                    <AiFillPlayCircle />
                    <span>Play</span>
                </button>
                <button
                    className={styles.queueButton}
                    onClick={() => addToQueue(episode)}
                >
                    <MdPlaylistAdd />
                    <span>Queue</span>
                </button>
            </div>
        </div>
    )
}

export default EpisodeListItem
