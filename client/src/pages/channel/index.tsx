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
import { keywordSelect } from "@/lib/search"
import { Episode } from "@/lib/types"

const ChannelPage = () => {
    const { id } = useParams()
    const { queueFromList } = usePlayer()

    const [queryFilter, setQueryFilter] = useState("")
    const [recentSort, setRecentSort] = useState("")

    const defaultValue = { channel: null, episodes: [] }
    const { data } = useQuery({
        queryKey: ["channel", id],
        queryFn: async () => {
            if (!id) throw Error("Invalid channel ID")
            return await getSubscriptionById(id)
        },
        staleTime: 60 * 1000 * 60 * 10,
        refetchOnReconnect: false,
        refetchOnWindowFocus: false,
    })
    const { channel, episodes } = data || defaultValue

    if (!channel) return <></>

    const sortEpisodes = (episodes: Episode[]) => {
        if (recentSort === "Most Recent") {
            return [...episodes].sort((a, b) => b.published - a.published)
        } else if (recentSort === "Least Recent") {
            return [...episodes].sort((a, b) => a.published - b.published)
        } else {
            return episodes
        }
    }

    const filteredEpisodes = sortEpisodes(
        keywordSelect(episodes, ["title", "description"], queryFilter),
    )

    return (
        <Layout>
            <ActionTitleBar
                actions={[
                    <SearchBar
                        text="Find episodes"
                        onSearch={setQueryFilter}
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
                            queueFromList(episodes)
                        }}
                    >
                        <BsFillPlayFill />
                        <span>Play All</span>
                    </Button>
                    <Select
                        items={["Most Recent", "Least Recent"]}
                        icon={<MdSort />}
                        onDone={setRecentSort}
                    />
                </div>
                <EpisodeList items={filteredEpisodes} withoutDate />
            </Layout>
        </Layout>
    )
}

export default ChannelPage
