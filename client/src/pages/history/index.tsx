import EpisodeList from "../../components/EpisodeList"
import SearchBar from "../../components/Search"
import Layout from "../../components/Layout"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import { useState } from "react"
import { clearHistory, getHistory } from "../../lib/api"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { Episode } from "@/lib/types"
import Button from "@/components/Button"
import { VscClearAll } from "react-icons/vsc"
import { keywordSelect } from "@/lib/search"

const HistoryPage = () => {
    const [showConfirmModal, setShowConfirmModal] = useState(false)
    const queryClient = useQueryClient()
    const clearMutation = useMutation({
        mutationFn: clearHistory,
        onSuccess: async () => {
            await queryClient.invalidateQueries({ queryKey: ["history"] })
        },
    })
    const { data, isLoading, isError } = useQuery({
        queryKey: ["history"],
        queryFn: getHistory,
    })
    const defaultValue: Episode[] = []
    const [query, setQuery] = useState("")
    const history = data
        ? keywordSelect(data, ["title", "description"], query)
        : defaultValue

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Listen History"
                    actions={[
                        <Button onClick={() => setShowConfirmModal(true)}>
                            <VscClearAll />
                            <span>Clear History</span>
                        </Button>,
                        <SearchBar text="Find episodes" onSearch={setQuery} />,
                    ]}
                />
                <EpisodeList withThumbnail items={history} />
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
                        onDone={() => {
                            clearMutation
                                .mutateAsync()
                                .then()
                                .catch(console.error)
                        }}
                    />
                )}
            </Layout>
        </Layout>
    )
}

export default HistoryPage
