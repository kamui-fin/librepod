import EpisodeList, { Episode } from "../../components/EpisodeList"
import styles from "./style.module.scss"
import SearchBar from "../../components/Search"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import { useState } from "react"
import { useOutletContext } from "react-router-dom"
import { clearHistory } from "../../lib/api"

const HistoryPage = () => {
    const [showConfirmModal, setShowConfirmModal] = useState(false)
    const { history, subsById } = useOutletContext()
    const [currHistory, setHistory] = useState(history)
    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Listen History"
                    actions={[<SearchBar text="Find episodes" data={history} cmpKeys={["title", "description"]} onSearch={(filtered) => setHistory(filtered)}/>]}
                />
                <div
                    className={styles.clearHistory}
                    onClick={() => setShowConfirmModal(true)}
                >
                    <span>Clear History</span>
                </div>
                <EpisodeList items={currHistory} channels={subsById} />
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
                        onDone={async () => {
                            const res = clearHistory()
                            console.log(res)
                            setHistory([])
                        }}
                    />
                )}
            </Layout>
        </Layout>
    )
}

export default HistoryPage
