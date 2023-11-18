import Button from "../../components/Button"
import Layout from "../../components/Layout"
import SearchBar from "../../components/Search"
import { BsCalendarWeek, BsFillPlayFill } from "react-icons/bs"
import EpisodeList from "../../components/EpisodeList"
import ActionTitleBar from "../../components/ActionTitleBar"
import Select from "../../components/Select"
import { DateTime } from "luxon"
import { usePlayer } from "@/lib/usePlayer"
import { useQuery } from "@tanstack/react-query"
import { getFeed } from "@/lib/api"
import { ChannelEpisode, Episode } from "@/lib/types"
import { useState } from "react"

const FeedPage = () => {
    const { data } = useQuery({
        queryKey: ['feed'],
        queryFn: getFeed
    })
    const defaultValue: ChannelEpisode[] = [];
    const feed = data || defaultValue
    const [foundEntries, setFoundEntries] = useState<ChannelEpisode[]>(feed)

    const { queueFromList } = usePlayer()

    const getEpisodeDateDiff = (ep: Episode) =>
        DateTime.fromJSDate(new Date(ep.published / 1000))
            .diff(DateTime.now(), ["days", "months", "year", "hours"])
            .toObject()

    const filterEpisodes = (text: string) => {
        switch (text) {
            case "Today":
                setFoundEntries(
                    feed.filter((ep) => {
                        const diff = getEpisodeDateDiff(ep.episode)
                        return (
                            diff.years === 0 &&
                            diff.months === 0 &&
                            diff.days === 0 &&
                            diff.hours! >= -24
                        )
                    }),
                )
                break;
            case "2 Weeks":
                setFoundEntries(
                    feed.filter((ep) => {
                        const diff = getEpisodeDateDiff(ep.episode)
                        return (
                            diff.years === 0 &&
                            diff.months === 0 &&
                            diff.days! >= -14
                        )
                    }),
                )
                break
            case "Past Month":
                setFoundEntries(
                    feed.filter((ep) => {
                        const diff = getEpisodeDateDiff(ep.episode)
                        return diff.years === 0 && diff.months! >= -1
                    }),
                )
                break
            case "All Time":
                setFoundEntries(feed)
                break
            default:
                break
        }
    }

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Feed"
                    actions={[
                        <Button
                            secondary
                            onClick={() => {
                                queueFromList(foundEntries.map(entry => entry.episode))
                            }}
                        >
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
                            data={foundEntries}
                            cmpKeys={["title", "description"]}
                            onSearch={(data) => setFoundEntries(data)}
                        />,
                    ]}
                />
                <EpisodeList items={foundEntries} />
            </Layout>
        </Layout>
    )
}

export default FeedPage
