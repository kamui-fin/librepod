import EpisodeList, { Episode } from "../../components/EpisodeList"
import styles from "./style.module.scss"
import SearchBar from "../../components/Search"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import { useState } from "react"

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

const HistoryPage = () => {
    const [showConfirmModal, setShowConfirmModal] = useState(false)
    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Listen History"
                    actions={[<SearchBar text="Find episodes" />]}
                />
                <div
                    className={styles.clearHistory}
                    onClick={() => setShowConfirmModal(true)}
                >
                    <span>Clear History</span>
                </div>
                <EpisodeList items={data} />
                {showConfirmModal && (
                    <Modal
                        title="Clear History"
                        content={
                            <p>
                                Are you sure that you want to clear your
                                history? All data will be irreversibly lost.
                            </p>
                        }
                        actionName="Delete"
                        primary={true}
                        open={showConfirmModal}
                        setOpen={setShowConfirmModal}
                    />
                )}
            </Layout>
        </Layout>
    )
}

export default HistoryPage
