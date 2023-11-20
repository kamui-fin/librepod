import Button from "../../components/Button"
import Layout from "../../components/Layout"
import SearchBar from "../../components/Search"
import SubscriptionCard from "../../components/SubscriptionCard"
import styles from "./style.module.scss"
import { IoMdAdd } from "react-icons/io"
import ActionTitleBar from "../../components/ActionTitleBar"
import Modal from "../../components/Modal"
import Input from "../../components/Input"
import { useState } from "react"
import { addSubscription, getSubscriptions } from "../../lib/api"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { keywordSelect } from "@/lib/search"

const SubscriptionsPage = () => {
    const [showAddModal, setShowAddModal] = useState(false)
    const [addRssLink, setAddRssLink] = useState("")

    const queryClient = useQueryClient()
    const addMutation = useMutation({
        mutationFn: addSubscription,
        onSuccess: async (channel) => {
            await queryClient.invalidateQueries({ queryKey: ["subscriptions"] })
            await queryClient.invalidateQueries({ queryKey: ["feed"] })
        },
    })
    const { data, isLoading, isError } = useQuery({
        queryKey: ["subscriptions"],
        queryFn: getSubscriptions,
    })

    const [filter, setFilter] = useState("")
    const subscriptions = data
        ? keywordSelect(data, ["title", "description"], filter)
        : []

    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Subscriptions"
                    actions={[
                        <Button onClick={() => setShowAddModal(true)}>
                            <IoMdAdd />
                        </Button>,
                        <SearchBar
                            text="Search channels"
                            onSearch={setFilter}
                        />,
                    ]}
                />
                <div className={styles.subs}>
                    {!subscriptions.length ? (
                        <p>No channels found.</p>
                    ) : (
                        subscriptions.map((currSub) => (
                            <SubscriptionCard
                                sub={currSub}
                                onDelete={async () => {
                                    await queryClient.invalidateQueries({
                                        queryKey: ["subscriptions"],
                                    })
                                }}
                            />
                        ))
                    )}
                </div>
                <Modal
                    title="Add Subscription"
                    content={
                        <Input
                            onChange={(e) => setAddRssLink(e.target.value)}
                            placeholder="RSS Link"
                            value={addRssLink}
                        />
                    }
                    actionName="Add"
                    primary={false}
                    open={showAddModal}
                    setOpen={setShowAddModal}
                    onDone={() => {
                        addMutation
                            .mutateAsync(addRssLink)
                            .then()
                            .catch(console.error)
                    }}
                />
            </Layout>
        </Layout>
    )
}

export default SubscriptionsPage
