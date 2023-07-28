import { Link } from "react-router-dom"
import styles from "./style.module.scss"
import { stripHtml } from "string-strip-html"
import { AiFillPlayCircle } from "react-icons/ai"
import { Episode, Subscription } from "../../../lib/types"

interface Props {
    item: Episode
    channel?: Subscription
    channelOnly: boolean
}

const EpisodeListItem = ({ item, channel, channelOnly }: Props) => {
    return (
        <div className={styles.listItem}>
            {channelOnly ? (
                <div className={styles.meta}>
                    <span className={styles.channel}>
                        {new Date(item.published / 1000).toLocaleDateString()}
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
                                130
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
                <button>
                    <AiFillPlayCircle />
                    <span>28 min</span>
                </button>
            </div>
        </div>
    )
}

export default EpisodeListItem
