import { Outlet } from "react-router-dom"
import Sidebar from "../../components/Sidebar"
import styles from "./style.module.scss"
import { AiOutlineHome } from "react-icons/ai"
import {
    MdOutlineRssFeed,
    MdOutlineHistory,
    MdOutlineSettings,
    MdQueueMusic,
} from "react-icons/md"
import { PlayerProvider } from "../../lib/usePlayer"

const navItems = [
    {
        to: "/",
        icon: <AiOutlineHome />,
        text: "New Releases",
    },
    {
        to: "/subscriptions",
        icon: <MdOutlineRssFeed />,
        text: "Subscriptions",
    },
    {
        to: "/queue",
        icon: <MdQueueMusic />,
        text: "Queue",
    },
    {
        to: "/history",
        icon: <MdOutlineHistory />,
        text: "Play History",
    },
    {
        to: "/settings",
        icon: <MdOutlineSettings />,
        text: "Settings",
    },
]

const HomePage = () => {
    return (
        <PlayerProvider>
            <div className={styles.container}>
                <Sidebar navItems={navItems} />
                <Outlet />
            </div>
        </PlayerProvider>
    )
}

export default HomePage
