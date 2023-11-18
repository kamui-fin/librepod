import styles from "./style.module.scss"

import { BsFillPlayFill } from "react-icons/bs"
import EpisodeList from "../../components/EpisodeList"
import Button from "../../components/Button"
import SearchBar from "../../components/Search"
import Layout from "../../components/Layout"
import Select from "../../components/Select"
import cx from "classnames"
import { MdSort } from "react-icons/md"
import ActionTitleBar from "../../components/ActionTitleBar"
import Divider from "../../components/Divider"
import ChannelMeta from "../../components/ChannelMeta"
import { useParams } from "react-router-dom"
import { useState } from "react"
import { usePlayer } from "../../lib/usePlayer"
import { getSubscriptionById } from "@/lib/api"
import { useQuery } from "@tanstack/react-query"

const ChannelPage = () => {
    const { id } = useParams();
    const { queueFromList } = usePlayer()

    const defaultValue = { channel: null, episodes: []};
    const { data } = useQuery({
        queryKey: ['channel', id],
        queryFn: async () => {
            if (!id) throw Error("Invalid channel ID")
            return await getSubscriptionById(id)
        },
    })
    const { channel, episodes } = data || defaultValue;

    const [episodeData, setEpisodeData] = useState(episodes)

    if (!channel) return <></>

    return (
        <Layout>
            <ActionTitleBar
                actions={[
                    <SearchBar
                        text="Find episodes"
                        data={episodes}
                        cmpKeys={["title", "description"]}
                        onSearch={(filtered) => setEpisodeData(filtered)}
                    />,
                ]}
            />
            <Layout inner>
                <ChannelMeta channel={channel} />
                <p className={styles.epCount}>
                    {channel.num_episodes} EPISODES
                </p>
                <Divider />
                <div className={cx(styles.actions, styles.spaceBottom)}>
                    <Button
                        secondary
                        onClick={() => {
                            queueFromList(episodeData)
                        }}
                    >
                        <BsFillPlayFill />
                        <span>Play All</span>
                    </Button>
                    <Select
                        items={["Most Recent", "Least Recent"]}
                        icon={<MdSort />}
                        onDone={(text) => {
                            if (text === "Most Recent") {
                                setEpisodeData(
                                    [...episodeData].sort(
                                        (a, b) => b.published - a.published,
                                    ),
                                )
                            } else if (text === "Least Recent") {
                                setEpisodeData(
                                    [...episodeData].sort(
                                        (a, b) => a.published - b.published,
                                    ),
                                )
                            }
                        }}
                    />
                </div>
                <EpisodeList items={episodeData.map(episode => ({ episode }))} />
            </Layout>
        </Layout>
    )
}

export default ChannelPage
