import { Outlet } from "react-router-dom"
import Sidebar from "../../components/Sidebar"
import styles from "./style.module.scss"

const HomePage = () => {
    return (
        <div className={styles.container}>
            <Sidebar />
            <Outlet />
        </div>
    )
}

export default HomePage
