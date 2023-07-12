import Layout from "../../components/Layout"
import styles from "./style.module.scss"
import { GrCircleInformation } from "react-icons/gr"

const SettingsPage = () => {
    return (
        <Layout>
            <h1>Settings</h1>
            <hr className={styles.logoDivider} />
            <div className={styles.notImplemented}>
                <GrCircleInformation />
                <span>Coming soon!</span>
            </div>
        </Layout>
    )
}

export default SettingsPage
