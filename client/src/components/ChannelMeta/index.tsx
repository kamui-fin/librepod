import { BiLinkAlt } from "react-icons/bi"
import styles from "./style.module.scss"
import { Channel } from "../../lib/types"
import { stripHtml } from "string-strip-html"

interface Props {
    channel: Channel
}

const ChannelMeta = ({ channel }: Props) => {
    return (
        <div className={styles.channelMeta}>
            <div className={styles.image}>
                <img src={channel.image || ''} />
            </div>
            <div className={styles.textMeta}>
                <h1 className={styles.title}>{channel.title}</h1>
                <div className={styles.authors}>{channel.author}</div>
                <div className={styles.link}>
                    <BiLinkAlt />
                    <span>{channel.website_link}</span>
                </div>
                <div className={styles.desc}>
                    {stripHtml(channel.description || '').result}
                </div>
            </div>
        </div>
    )
}

export default ChannelMeta
