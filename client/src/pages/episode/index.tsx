import styles from "./style.module.scss"
import { Link, useParams } from "react-router-dom"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Divider from "../../components/Divider"
import parse from "html-react-parser"
import { AiFillPlayCircle } from "react-icons/ai"
import { getHumanDate } from "../../lib/utils"
import { MdPlaylistAdd } from "react-icons/md"
import { MdDownloadDone } from "react-icons/md"
import { usePlayer } from "../../lib/usePlayer"
import { getEpisodeById, markPlayed } from "@/lib/api"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import DropdownContextMenu from "@/components/DropdownContextMenu"
import Loader from "@/components/Loader"

const EpisodePage = () => {
    const { id } = useParams()
    const {
        data: episode,
        isLoading,
        error,
    } = useQuery({
        queryKey: ["episode", id],
        queryFn: async () => {
            if (!id) throw Error("Invalid episode ID")
            return await getEpisodeById(id)
        },
        staleTime: Infinity,
        refetchOnReconnect: false,
        refetchOnWindowFocus: false,
    })

    const queryClient = useQueryClient()
    const markPlayedMutation = useMutation({
        mutationFn: markPlayed,
        onSuccess: async (episode) => {
            await queryClient.invalidateQueries({ queryKey: ["history"] })
        },
    })

    const { addToQueue, addToFront } = usePlayer()

    return (
        <Layout>
            <ActionTitleBar />
            <Layout inner>
                <Loader isLoading={isLoading}>
                    {episode !== undefined && (
                        <>
                            <div className={styles.channel}>
                                <img src={episode.channel_image || ""} alt="" />
                                <div className={styles.text}>
                                    <Link
                                        to={`/subscriptions/channel/${episode.channel_id}`}
                                    >
                                        <h3>{episode.channel_title}</h3>
                                    </Link>
                                    <p>{getHumanDate(episode.published)}</p>
                                </div>
                            </div>
                            <h1 className={styles.episodeTitle}>
                                {episode.title}
                            </h1>
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
                                <DropdownContextMenu
                                    className={styles.dropdown}
                                    menuItemProps={[
                                        {
                                            icon: <MdDownloadDone />,
                                            text: "Mark Played",
                                            onClick: () => {
                                                markPlayedMutation
                                                    .mutateAsync(episode)
                                                    .then()
                                                    .catch(console.error)
                                            },
                                        },
                                    ]}
                                />
                            </div>
                            <Divider />
                            <div className={styles.content}>
                                {parse(
                                    episode.content ||
                                        episode.description ||
                                        "",
                                )}
                            </div>
                        </>
                    )}
                </Loader>
            </Layout>
        </Layout>
    )
}

export default EpisodePage
