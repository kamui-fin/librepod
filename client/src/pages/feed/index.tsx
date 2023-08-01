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
import { useEffect, useState } from "react"
import Select from "../../components/Select"
import { DateTime } from "luxon"
import { usePlayerContext } from "../../lib/usePlayer"

const FeedPage = () => {
    const { episodes, subsById } = useOutletContext()
    const { queueFromList } = usePlayerContext()
    const [episodeData, setEpisodeData] = useState(episodes)

    const getEpisodeDateDiff = (ep) => (
        DateTime.fromJSDate(
            new Date(
                ep.published / 1000
            )
        )
            .diff(DateTime.now(), [
                "days",
                "months",
                "year",
                "hours"
            ])
            .toObject()
    )


    const filterEpisodes = (text) => {
        switch (text) {
            case "Today":
                setEpisodeData(
                    episodes.filter((ep) => {
                        const diff = getEpisodeDateDiff(ep)
                        return (
                            diff.years === 0 &&
                            diff.months === 0 &&
                            diff.days === 0 &&
                            diff.hours >= -24
                        )
                    })
                )
            case "2 Weeks":
                setEpisodeData(
                    episodes.filter((ep) => {
                        const diff = getEpisodeDateDiff(ep)
                        return (
                            diff.years === 0 &&
                            diff.months === 0 &&
                            diff.days >= -14
                        )
                    })
                )
                break
            case "Past Month":
                setEpisodeData(
                    episodes.filter((ep) => {
                        const diff = getEpisodeDateDiff(ep)
                        return (
                            diff.years === 0 &&
                            diff.months >= -1
                        )
                    })
                )
                break
            case "All Time":
                setEpisodeData(episodes)
                break
        }
    }

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Feed"
                    actions={[
                        <Button secondary onClick={() => {
                            queueFromList(episodeData)
                        }}>
                            <BsFillPlayFill />
                            <span>Play All</span>
                        </Button>,
                        <Select
                            items={[
                                "Today",
                                "2 Weeks",
                                "Past Month",
                                "Past Year",
                                "All Time",
                            ]}
                            defaultIndex={4}
                            icon={<BsCalendarWeek />}
                            onDone={filterEpisodes}
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
