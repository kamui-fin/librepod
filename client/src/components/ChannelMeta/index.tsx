import { BiLinkAlt } from "react-icons/bi"
import { Episode } from "../EpisodeList"
import styles from "./style.module.scss"
import { Subscription } from "../../lib/types"
import { stripHtml } from "string-strip-html"

interface Props {
    channel: Subscription
}

const ChannelMeta = ({ channel }: Props) => {
    return (
        <div className={styles.channelMeta}>
            <div className={styles.image}>
                <img src={channel.logo.uri} />
            </div>
            <div className={styles.textMeta}>
                <h1 className={styles.title}>{channel.title}</h1>
                <div className={styles.authors}>
                    {channel.authors.map((p) => p.name).join(", ")}
                </div>
                <div className={styles.link}>
                    <BiLinkAlt />
                    <span>{channel.website_link}</span>
                </div>
                <div className={styles.desc}>
                    {stripHtml(channel.description.content).result}
                </div>
            </div>
        </div>
    )
}

export default ChannelMeta
