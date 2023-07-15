import { Outlet } from "react-router-dom"
import Sidebar from "../../components/Sidebar"
import styles from "./style.module.scss"
import { AiOutlineHome } from "react-icons/ai"
import {
    MdOutlineRssFeed,
    MdOutlineHistory,
    MdOutlineSettings,
} from "react-icons/md"

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
        <div className={styles.container}>
            <Sidebar navItems={navItems} />
            <Outlet />
        </div>
    )
}

export default HomePage
