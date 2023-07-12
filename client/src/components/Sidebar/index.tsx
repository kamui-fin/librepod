import styles from "./style.module.scss"
import LogoSvg from "@/assets/logo.svg"
import { NavLink } from "react-router-dom"
import { AiOutlineHome } from "react-icons/ai"
import {
    MdOutlineRssFeed,
    MdOutlineHistory,
    MdOutlineSettings,
    MdOutlineLogout,
} from "react-icons/md"
import { useAuth } from "../../lib/useAuth"
import { Link } from "react-router-dom"

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

const Sidebar = () => {
    const { logout } = useAuth()
    return (
        <aside className={styles.container}>
            <nav>
                <ul>
                    <li className={styles.logo}>
                        <img src={LogoSvg} />
                        <hr className={styles.logoDivider} />
                    </li>
                    <div className={styles.menu}>
                        {navItems.map(({ to, text, icon }) => (
                            <NavLink
                                to={to}
                                className={({ isActive }) =>
                                    isActive ? styles.active : ""
                                }
                            >
                                {icon}
                                <li>{text}</li>
                            </NavLink>
                        ))}
                        <a className={styles.logout} onClick={logout}>
                            <MdOutlineLogout />
                            <span>Logout</span>
                        </a>
                    </div>
                </ul>
            </nav>
        </aside>
    )
}

export default Sidebar
