import styles from "./style.module.scss"
import { ChannelEpisode, Episode } from "../../lib/types"
import { Link, useLoaderData } from "react-router-dom"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Divider from "../../components/Divider"
import parse from "html-react-parser"
import { AiFillPlayCircle } from "react-icons/ai"
import { getHumanDate } from "../../lib/utils"
import ContextMenu from "../../components/ContextMenu"
import { MdPlaylistAdd } from "react-icons/md"

const EpisodePage = () => {
    const { episode, channel }: ChannelEpisode = useLoaderData()

    return (
        <Layout>
            <ActionTitleBar />
            <Layout inner>
                <div className={styles.channel}>
                    <img src={channel.channel.image} alt="" />
                    <div className={styles.text}>
                        <Link
                            to={`/subscriptions/channel/${channel.channel.id}`}
                        >
                            <h3>{channel.channel.title}</h3>
                        </Link>
                        <p>{getHumanDate(episode.published)}</p>
                    </div>
                </div>
                <h1 className={styles.episodeTitle}>{episode.title}</h1>
                <div className={styles.buttons}>
                    <div className={styles.play}>
                        <button>
                            <AiFillPlayCircle />
                            <span>28 min</span>
                        </button>
                    </div>
                    <div className={styles.iconButton}>
                        <MdPlaylistAdd />
                    </div>
                    <ContextMenu />
                </div>
                <Divider />
                <div className={styles.content}>{parse(episode.content)}</div>
            </Layout>
        </Layout>
    )
}

export default EpisodePage
