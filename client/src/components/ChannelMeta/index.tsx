import { BiLinkAlt } from "react-icons/bi";
import { Episode } from "../EpisodeList";
import styles from "./style.module.scss"

export interface Channel {
    title: string;
    description: string;
    link: string;
    authors: string;
    image: string;
    episodes: Episode[];
}

interface Props {
    channel: Channel;
}

const ChannelMeta = ({ channel }: Props) => {
    return (
        <div className={styles.channelMeta}>
            <div className={styles.image}>
                <img src={channel.image} />
            </div>
            <div className={styles.textMeta}>
                <h1 className={styles.title}>{channel.title}</h1>
                <div className={styles.authors}>{channel.authors}</div>
                <div className={styles.link}>
                    <BiLinkAlt />
                    <span>{channel.link}</span>
                </div>
                <div className={styles.desc}>
                    {channel.description}
                </div>
            </div>
        </div>
    )
}

export default ChannelMeta
