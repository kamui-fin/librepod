import { Outlet, useLoaderData } from "react-router-dom"
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
import { useQuery } from "@tanstack/react-query"
import { feedLoader } from "@/lib/api"

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
    const { data }  = useQuery({
        queryKey: ['feed'],
        queryFn: () => feedLoader
    })
    return (
        <PlayerProvider>
            <div className={styles.container}>
                <Sidebar test={data} navItems={navItems} />
                <Outlet context={data} />
            </div>
        </PlayerProvider>
    )
}

export default HomePage
