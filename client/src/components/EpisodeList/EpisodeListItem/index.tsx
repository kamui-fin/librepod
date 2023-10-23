import { Link } from "react-router-dom"
import styles from "./style.module.scss"
import { stripHtml } from "string-strip-html"
import { AiFillPlayCircle } from "react-icons/ai"
import { Episode, Subscription } from "../../../lib/types"
import { getHumanDate } from "../../../lib/utils"
import { MdPlaylistAdd } from "react-icons/md"
import { usePlayerContext } from "../../../lib/usePlayer"

interface Props {
    item: Episode
    channel?: Subscription
    channelOnly: boolean
}

const EpisodeListItem = ({ item, channel, channelOnly }: Props) => {
    const { addToQueue, addToFront } = usePlayerContext()
    return (
        <div className={styles.listItem}>
            {channelOnly ? (
                <div className={styles.meta}>
                    <span className={styles.channel}>
                        {getHumanDate(item.published)}
                    </span>
                    <Link to={`/episode/${item.id}`}>
                        <h3 className={styles.title}>{item.title}</h3>
                    </Link>
                    <p className={styles.desc}>
                        {stripHtml(item.description).result.substring(0, 150) +
                            "..."}
                    </p>
                </div>
            ) : (
                <div className={styles.withChannel}>
                    <div className={styles.image}>
                        <img src={channel.image} />
                    </div>
                    <div className={styles.meta}>
                        <Link to={`/episode/${item.id}`}>
                            <h3 className={styles.title}>{item.title}</h3>
                        </Link>
                        <p className={styles.desc}>
                            {stripHtml(item.description).result.substring(
                                0,
                                130,
                            ) + "..."}
                        </p>
                        <Link to={`/subscriptions/channel/${item.channel_id}`}>
                            <span className={styles.channel}>
                                {channel.title}
                            </span>
                        </Link>
                    </div>
                </div>
            )}
            <div className={styles.play}>
                <button
                    className={styles.playButton}
                    onClick={() => addToFront(item)}
                >
                    <AiFillPlayCircle />
                    <span>Play</span>
                </button>
                <button
                    className={styles.queueButton}
                    onClick={() => addToQueue(item)}
                >
                    <MdPlaylistAdd />
                    <span>Queue</span>
                </button>
            </div>
        </div>
    )
}

export default EpisodeListItem
