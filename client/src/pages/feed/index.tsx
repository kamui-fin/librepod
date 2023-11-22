import Button from "../../components/Button"
import Layout from "../../components/Layout"
import SearchBar from "../../components/Search"
import { BsCalendarWeek, BsFillPlayFill } from "react-icons/bs"
import EpisodeList from "../../components/EpisodeList"
import ActionTitleBar from "../../components/ActionTitleBar"
import Select from "../../components/Select"
import { DateTime } from "luxon"
import { usePlayer } from "@/lib/usePlayer"
import { useInfiniteQuery } from "@tanstack/react-query"
import { getFeed } from "@/lib/api"
import { Episode } from "@/lib/types"
import { useEffect, useState } from "react"
import { keywordSelect } from "@/lib/search"
import Loader from "@/components/Loader"
import { useInView } from "react-intersection-observer"
import cx from "classnames"
import styles from "./style.module.scss"

const RESULTS_PER_PAGE = 15

type Params = { pageParam?: number }
type KeyParams = {
    [key: string]: any
}

const FeedPage = () => {
    const { data, isLoading, fetchNextPage, hasNextPage } = useInfiniteQuery<
        Episode[],
        Episode,
        Episode[],
        Array<string | KeyParams>,
        number
    >({
        queryKey: ["feed"],
        queryFn: async ({ pageParam = 0 }: Params): Promise<Episode[]> =>
            await getFeed({
                offset: RESULTS_PER_PAGE * pageParam,
                limit: RESULTS_PER_PAGE,
            }),
        getNextPageParam: (lastPage, allPages) => {
            return lastPage.length > 0 ? allPages.length + 1 : undefined
        },
        initialPageParam: 0,
        staleTime: 60 * 1000 * 60 * 10,
        refetchOnReconnect: false,
        refetchOnWindowFocus: false,
        select: (data) => data.pages.flat(),
    })
    const { ref, inView } = useInView()
    useEffect(() => {
        if (inView) void fetchNextPage()
    }, [fetchNextPage, inView])

    const { queueFromList } = usePlayer()

    const getEpisodeDateDiff = (ep: Episode) =>
        DateTime.fromJSDate(new Date(ep.published / 1000))
            .diff(DateTime.now(), ["days", "months", "year", "hours"])
            .toObject()

    const [dateFilter, setDateFilter] = useState("All Time")
    const [queryFilter, setQueryFilter] = useState("")

    const filterEpisodesByDate = (episodes: Episode[]) => {
        switch (dateFilter) {
            case "Today":
                return episodes.filter((ep) => {
                    const diff = getEpisodeDateDiff(ep)
                    return (
                        diff.years === 0 &&
                        diff.months === 0 &&
                        diff.days === 0 &&
                        diff.hours! >= -24
                    )
                })
            case "2 Weeks":
                return episodes.filter((ep) => {
                    const diff = getEpisodeDateDiff(ep)
                    return (
                        diff.years === 0 &&
                        diff.months === 0 &&
                        diff.days! >= -14
                    )
                })
            case "Past Month":
                return episodes.filter((ep) => {
                    const diff = getEpisodeDateDiff(ep)
                    return diff.years === 0 && diff.months! >= -1
                })
            case "All Time":
                return episodes
            default:
                return episodes
        }
    }

    const filterEpisodes = (episodes: Episode[]) =>
        keywordSelect(
            filterEpisodesByDate(episodes),
            ["title", "description"],
            queryFilter,
        )
    const feed = data ? filterEpisodes(data) : []

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Feed"
                    actions={[
                        <Button
                            secondary
                            onClick={() => {
                                queueFromList(feed)
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
                            onDone={setDateFilter}
                        />,
                        <SearchBar
                            text="Find episodes"
                            onSearch={setQueryFilter}
                        />,
                    ]}
                />
                <Loader isLoading={isLoading}>
                    <EpisodeList withThumbnail items={feed} />
                    <Loader
                        className={cx({ [styles.hidden]: !hasNextPage })}
                        isLoading
                        ref={ref}
                    />
                </Loader>
            </Layout>
        </Layout>
    )
}

export default FeedPage
