import styles from "./style.module.scss"

import { BsFillPlayFill } from "react-icons/bs";
import EpisodeList, { Episode } from "../../components/EpisodeList";
import Button from "../../components/Button";
import SearchBar from "../../components/Search";
import Layout from "../../components/Layout";
import Select from "../../components/Select";
import cx from "classnames"
import { MdSort } from "react-icons/md";
import ActionTitleBar from "../../components/ActionTitleBar";
import Divider from "../../components/Divider";
import ChannelMeta, { Channel } from "../../components/ChannelMeta";

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

const channel: Channel = {
    image: "https://fdfs.xmcdn.com/group41/M05/A9/5D/wKgJ8lrELXrw9POyAACuHutlc7A709.jpg",
    title: "科技乱炖",
    authors: "郑长明, 哲瀚",
    link: "https://www.ximalaya.com/album/70159551",
    description: "由多名资深从业者主持的科技点评播客，以实际工作中积累的经验为基础，结合实际，把近期科技热点变成犀利、独到、深刻的独家观点。",
    episodes: data
}

const ChannelPage = () => {
    return (
        <Layout>
            <ActionTitleBar actions={[
                <SearchBar text="Find episodes" />
            ]} />
            <Layout inner>
                <ChannelMeta channel={channel} />
                <p className={styles.epCount}>
                    {channel.episodes.length} EPISODES
                </p>
                <Divider />
                <div className={cx(styles.actions, styles.spaceBottom)}>
                    <Button secondary>
                        <BsFillPlayFill />
                        <span>Play All</span>
                    </Button>
                    <Select items={["Most Recent", "Least Recent"]} icon={<MdSort />} />
                </div>
                <EpisodeList items={channel.episodes} channelOnly />
            </Layout>
        </Layout>
    )
}

export default ChannelPage
