import styles from "./style.module.scss"
import LogoSvg from "@/assets/logo.svg"
import { NavLink } from "react-router-dom"
import { MdOutlineLogout } from "react-icons/md"
import { useAuth } from "../../lib/useAuth"
import SidebarPlayer from "../SidebarPlayer"

interface NavItem {
    to: string;
    icon: React.ReactNode;
    text: string;
}

interface Props {
    navItems: NavItem[];
}

const Sidebar = ({ navItems }: Props) => {
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
                <SidebarPlayer />
            </nav>
        </aside>
    )
}

export default Sidebar
