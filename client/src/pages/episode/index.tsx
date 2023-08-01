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
import { usePlayerContext } from "../../lib/usePlayer"
import { axios } from "../../lib/api"

const EpisodePage = () => {
    const { episode, channel }: ChannelEpisode = useLoaderData()
    const { addToQueue, addToFront } = usePlayerContext()

    const markPlayed = (episode) => {
        axios.post(`/history/${episode.id}`)
    }

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
                        <button onClick={() => addToFront(episode)}>
                            <AiFillPlayCircle />
                            <span>Play</span>
                        </button>
                    </div>
                    <div className={styles.iconButton} onClick={() => addToQueue(episode)}>
                        <MdPlaylistAdd />
                    </div>
                    <ContextMenu menuItems={[{ text: "Mark Played", handler: () => markPlayed(episode) }]} />
                </div>
                <Divider />
                <div className={styles.content}>{parse(episode.content)}</div>
            </Layout>
        </Layout>
    )
}

export default EpisodePage
