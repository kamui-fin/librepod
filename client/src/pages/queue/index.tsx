import React, { useState } from "react"
import styles from "./style.module.scss"
import Layout from "../../components/Layout"
import SearchBar from "../../components/Search"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import EpisodeList from "../../components/EpisodeList"
import { usePlayerContext } from "../../lib/usePlayer"
import { useOutletContext } from "react-router-dom"

const QueuePage = () => {
    const [showConfirmModal, setShowConfirmModal] = useState(false)
    const { queue, clearQueue } = usePlayerContext()
    const { subsById } = useOutletContext()

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar title="Queue" />
                <div
                    className={styles.clearHistory}
                    onClick={() => setShowConfirmModal(true)}
                >
                    <span>Clear Queue</span>
                </div>
                <EpisodeList
                    withoutDate={true}
                    items={queue}
                    channels={subsById}
                />
                {showConfirmModal && (
                    <Modal
                        title="Clear Queue"
                        content={
                            <p>
                                Are you sure that you want to clear the Queue?
                            </p>
                        }
                        actionName="Delete"
                        primary={true}
                        open={showConfirmModal}
                        setOpen={setShowConfirmModal}
                        onDone={clearQueue}
                    />
                )}
            </Layout>
        </Layout>
    )
}

export default QueuePage