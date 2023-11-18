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
import { Channel } from "../../lib/types"
import { addSubscription, getSubscriptions } from "../../lib/api"
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

const SubscriptionsPage = () => {
    const [showAddModal, setShowAddModal] = useState(false)
    const [addRssLink, setAddRssLink] = useState("")

    const queryClient = useQueryClient()
    const addMutation = useMutation({
        mutationFn: addSubscription,
        onSuccess: async () => {
            await queryClient.invalidateQueries({ queryKey: ['subscriptions'] })
        }
    })
    const { data, isLoading, isError } = useQuery({
        queryKey: ['subscriptions'],
        queryFn: getSubscriptions,
    })
    const defaultValue: Channel[] = [];
    const subscriptionsById = data || defaultValue
    const [foundSubs, setFoundSubs] = useState<Channel[]>(subscriptionsById)

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
                            data={Object.values(subscriptionsById)}
                            cmpKeys={["title", "description"]}
                            onSearch={(filtered) => setFoundSubs(filtered)}
                        />,
                    ]}
                />
                <div className={styles.subs}>
                    {!foundSubs.length ? (
                        <p>No channels found.</p>
                    ) : (
                        foundSubs.map((currSub) => (
                            <SubscriptionCard
                                sub={currSub}
                                onDelete={() => {
                                    setFoundSubs(foundSubs.filter((otherSub) => otherSub.id !== currSub.id))
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
                    onDone={() =>  {
                        addMutation.mutateAsync(addRssLink).then().catch(console.error);
                    }}
                />
            </Layout>
        </Layout>
    )
}

export default SubscriptionsPage
