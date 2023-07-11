import styles from "./style.module.scss"
import LogoSvg from "@/assets/logo.svg"
import { NavLink } from "react-router-dom";

const navItems = [
    {
        to: "/feed",
        text: "New Releases"
    },
    {
        to: "/subscriptions",
        text: "Subscriptions"
    },
    {
        to: "/history",
        text: "History"
    },
    {
        to: "/settings",
        text: "Settings"
    }
]

const Sidebar = () => {
    return (
        <aside className={styles.container}>
            <nav>
                <ul>
                    <li className={styles.logo} >
                        <img src={LogoSvg} />
                        <hr className={styles.logoDivider} />
                    </li>
                    <div className={styles.menu}>
                        {navItems.map(({ to, text }) => (
                            <NavLink to={to}
                                className={({ isActive }) =>
                                    isActive
                                    && "active"
                                }
                            >
                                <li>
                                    {text}
                                </li>
                            </NavLink>
                        ))}
                    </div>
                </ul>
            </nav>
        </aside>
    )
}

export default Sidebar
