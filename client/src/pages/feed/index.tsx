import Button from "../../components/Button"
import Layout from "../../components/Layout"
import styles from "./style.module.scss"
import SearchBar from "../../components/Search"
import { BsFillPlayFill } from "react-icons/bs"
import EpisodeList, { Episode } from "../../components/EpisodeList"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"

const data: Episode[] = [
    {
        img: "https://fdfs.xmcdn.com/group53/M08/04/7F/wKgLfFxFi1DyK_sOAAIVVH_yP2g776.jpg",
        title: "4 Tips for Recruiting Top Talent to Join Your Team",
        description:
            "If you’re stretched thin and having trouble recruiting the right people to join your team, it can feel like an uphill battle. You have to keep doing the work at hand while taking on the tim...",
        channelName: "纽约文化沙龙",
        date: new Date(),
        duration: "28 min",
    },
    {
        img: "https://fdfs.xmcdn.com/group53/M08/04/7F/wKgLfFxFi1DyK_sOAAIVVH_yP2g776.jpg",
        title: "4 Tips for Recruiting Top Talent to Join Your Team",
        description:
            "If you’re stretched thin and having trouble recruiting the right people to join your team, it can feel like an uphill battle. You have to keep doing the work at hand while taking on the tim...",
        channelName: "纽约文化沙龙",
        date: new Date(new Date().setDate(new Date().getDate() - 5)),
        duration: "28 min",
    },
    {
        img: "https://fdfs.xmcdn.com/group53/M08/04/7F/wKgLfFxFi1DyK_sOAAIVVH_yP2g776.jpg",
        title: "4 Tips for Recruiting Top Talent to Join Your Team",
        description:
            "If you’re stretched thin and having trouble recruiting the right people to join your team, it can feel like an uphill battle. You have to keep doing the work at hand while taking on the tim...",
        channelName: "纽约文化沙龙",
        date: new Date(new Date().setDate(new Date().getDate() - 10)),
        duration: "28 min",
    },
]

const FeedPage = () => {
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
                        <SearchBar text="Find episodes" />,
                    ]}
                />
                <EpisodeList items={data} />
            </Layout>
            <Modal></Modal>
        </Layout>
    )
}

export default FeedPage
