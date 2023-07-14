import { BsFillPlayFill } from "react-icons/bs";
import EpisodeList, { Episode } from "../../components/EpisodeList";
import styles from "./style.module.scss"
import Button from "../../components/Button";
import SearchBar from "../../components/Search";
import Layout from "../../components/Layout";
import { BiArrowBack, BiLinkAlt } from "react-icons/bi"
import Select from "../../components/Select";
import cx from "classnames"
import { MdSort } from "react-icons/md";
import { useNavigate } from "react-router-dom";

interface Channel {
    title: string;
    description: string;
    link: string;
    authors: string;
    image: string;
    episodes: Episode[];
}

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
    const navigate = useNavigate();
    const goBack = () => {
        navigate(-1)
    }
    return (
        <Layout>
            <header>
                <div className={styles.back} onClick={goBack}>
                    <BiArrowBack />
                    <span>Back</span>
                </div>
                <div className={styles.actions}>
                    <SearchBar text="Find episodes" />
                </div>
            </header>
            <hr className={styles.logoDivider} />
            <div className={styles.channelMeta}>
                <div className={styles.image}>
                    <img src={channel.image} />
                </div>
                <div className={styles.textMeta}>
                    <h1 className={styles.title}>{channel.title}</h1>
                    <div className={styles.authors}>{channel.authors}</div>
                    <div className={styles.back}>
                        <BiLinkAlt />
                        <span>{channel.link}</span>
                    </div>
                    <div className={styles.desc}>
                        {channel.description}
                    </div>
                </div>
            </div>
            <div className={styles.container}>
                <p className={styles.epCount}>
                    {channel.episodes.length} EPISODES
                </p>
                <hr className={styles.logoDivider} />
                <div className={cx(styles.actions, styles.spaceBottom)}>
                    <Button secondary>
                        <BsFillPlayFill />
                        <span>Play All</span>
                    </Button>
                    <Select items={["Sort By"]} icon={<MdSort />} />
                </div>
                <EpisodeList items={channel.episodes} channelOnly />
            </div>
        </Layout>
    )
}

export default ChannelPage
