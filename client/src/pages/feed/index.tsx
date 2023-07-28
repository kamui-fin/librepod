import Button from "../../components/Button"
import Layout from "../../components/Layout"
import styles from "./style.module.scss"
import SearchBar from "../../components/Search"
import { BsCalendarWeek, BsFillPlayFill } from "react-icons/bs"
import EpisodeList from "../../components/EpisodeList"
import ActionTitleBar from "../../components/ActionTitleBar"
import ConfirmationModal from "../../components/Modal"
import { useLoaderData, useOutletContext } from "react-router-dom"
import { Episode } from "../../lib/types"
import { useState } from "react"
import Select from "../../components/Select"

const FeedPage = () => {
    const { episodes, subsById } = useOutletContext()
    const [episodeData, setEpisodeData] = useState(episodes)
    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Feed"
                    actions={[
                        <Button secondary>
                            <BsFillPlayFill />
                            <span>Play All</span>
                        </Button>,
                        <Select
                            items={[
                                "2 Weeks",
                                "Past Month",
                                "Past Year",
                                "All Time",
                            ]}
                            icon={<BsCalendarWeek />}
                            onDone={(text) => {}}
                        />,
                        <SearchBar
                            text="Find episodes"
                            data={episodes}
                            cmpKeys={["title", "description"]}
                            onSearch={(data) => setEpisodeData(data)}
                        />,
                    ]}
                />
                <EpisodeList items={episodeData} channels={subsById} />
            </Layout>
            <ConfirmationModal></ConfirmationModal>
        </Layout>
    )
}

export default FeedPage
