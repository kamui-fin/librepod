import ActionTitleBar from "../../components/ActionTitleBar"
import Layout from "../../components/Layout"
import styles from "./style.module.scss"
import { GrCircleInformation } from "react-icons/gr"

const SettingsPage = () => {
    return (
        <Layout>
            <ActionTitleBar title="Settings" />
            <div className={styles.notImplemented}>
                <GrCircleInformation />
                <span>Coming soon!</span>
            </div>
        </Layout>
    )
}

export default SettingsPage
