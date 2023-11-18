import EpisodeList from "../../components/EpisodeList"
import styles from "./style.module.scss"
import SearchBar from "../../components/Search"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import { useState } from "react"
import { clearHistory, getHistory } from "../../lib/api"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { ChannelEpisode } from "@/lib/types"

const HistoryPage = () => {
    const [showConfirmModal, setShowConfirmModal] = useState(false)
    const queryClient = useQueryClient()
    const clearMutation = useMutation({
        mutationFn: clearHistory,
        onSuccess: async () => {
            await queryClient.invalidateQueries({ queryKey: ['history'] })
        }
    })
    const { data, isLoading, isError } = useQuery({
        queryKey: ['history'],
        queryFn: getHistory,
    })
    const defaultValue: ChannelEpisode[] = [];
    const history = data || defaultValue
    const [foundEntries, setFoundEntries] = useState<ChannelEpisode[]>(history)
    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Listen History"
                    actions={[
                        <SearchBar
                            text="Find episodes"
                            data={history}
                            cmpKeys={["title", "description"]}
                            onSearch={(filtered) => setFoundEntries(filtered)}
                        />,
                    ]}
                />
                <div
                    className={styles.clearHistory}
                    onClick={() => setShowConfirmModal(true)}
                >
                    <span>Clear History</span>
                </div>
                <EpisodeList items={foundEntries} />
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
                        onDone={() => { clearMutation.mutateAsync().then().catch(console.error); }}
                    />
                )}
            </Layout>
        </Layout>
    )
}

export default HistoryPage
