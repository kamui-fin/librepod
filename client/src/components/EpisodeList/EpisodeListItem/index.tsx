import { Link } from "react-router-dom"
import styles from "./style.module.scss"
import { stripHtml } from "string-strip-html"
import { AiFillPlayCircle } from "react-icons/ai"
import { Episode } from "../../../lib/types"

interface Props {
    item: Episode
    channelOnly: boolean
}

const EpisodeListItem = ({ item, channelOnly }: Props) => {
    return (
        <div className={styles.listItem}>
            {channelOnly ? (
                <div className={styles.meta}>
                    <span className={styles.channel}>
                        {new Date(item.published / 1000).toLocaleDateString()}
                    </span>
                    <h3 className={styles.title}>{item.title}</h3>
                    <p className={styles.desc}>
                        {stripHtml(item.summary.content).result.replace(
                            /(.{50})..+/,
                            "$1&hellip;"
                        )}
                    </p>
                </div>
            ) : (
                <>
                    <div className={styles.image}>
                        <img src={item.channel.logo.uri} />
                    </div>
                    <div className={styles.meta}>
                        <h3 className={styles.title}>{item.title}</h3>
                        <p className={styles.desc}>
                            {stripHtml(item.summary.content).result.substring(
                                0,
                                130
                            ) + "..."}
                        </p>
                        <Link to={`/subscriptions/channel/${item.source_id}`}>
                            <span className={styles.channel}>
                                {item.channel.title}
                            </span>
                        </Link>
                    </div>
                </>
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
