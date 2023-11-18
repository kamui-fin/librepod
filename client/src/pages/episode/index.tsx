import styles from "./style.module.scss"
import { Link, useParams } from "react-router-dom"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Divider from "../../components/Divider"
import parse from "html-react-parser"
import { AiFillPlayCircle } from "react-icons/ai"
import { getHumanDate } from "../../lib/utils"
import ContextMenu from "../../components/ContextMenu"
import { MdPlaylistAdd } from "react-icons/md"
import { usePlayer} from "../../lib/usePlayer"
import { getEpisodeById, markPlayed } from "@/lib/api"
import { useQuery } from "@tanstack/react-query"

const EpisodePage = () => {
    const { id } = useParams()
    const defaultValue = {
        episode: null,
        channel: null,
    }
    const { data = defaultValue, isLoading } = useQuery({
        queryKey: ['episode', id],
        queryFn: async () => {
            if (!id) throw Error("Invalid episode ID")
            return await getEpisodeById(id)
        }, 
    })
    const { channel, episode } = data;
    const { addToQueue, addToFront } = usePlayer()
    
    if (!channel && !episode) {
        return <></>
    }

    return (
        <Layout>
            <ActionTitleBar />
            <Layout inner>
                <div className={styles.channel}>
                    <img src={channel?.image || ''} alt="" />
                    <div className={styles.text}>
                        <Link
                            to={`/subscriptions/channel/${channel.id}`}
                        >
                            <h3>{channel.title}</h3>
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
                    <div
                        className={styles.iconButton}
                        onClick={() => addToQueue(episode)}
                    >
                        <MdPlaylistAdd />
                    </div>
                    <ContextMenu
                        menuItems={[
                            {
                                text: "Mark Played",
                                handler: () => { markPlayed(episode).then().catch(console.error) } ,
                            },
                        ]}
                    />
                </div>
                <Divider />
                <div className={styles.content}>{parse(episode.content || '')}</div>
            </Layout>
        </Layout>
    )
}

export default EpisodePage
