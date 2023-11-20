import React, { useState } from "react"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import EpisodeList from "../../components/EpisodeList"
import { usePlayer } from "../../lib/usePlayer"
import Button from "@/components/Button"
import { VscClearAll } from "react-icons/vsc"

const QueuePage = () => {
    const [showConfirmModal, setShowConfirmModal] = useState(false)
    const { queue, clearQueue } = usePlayer()

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Queue"
                    actions={[
                        <Button onClick={() => setShowConfirmModal(true)}>
                            <VscClearAll />
                            <span>Clear Queue</span>
                        </Button>,
                    ]}
                />
                <EpisodeList withoutDate={true} withThumbnail items={queue} />
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
