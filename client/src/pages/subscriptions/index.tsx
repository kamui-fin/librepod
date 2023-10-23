import { Link, useLoaderData, useOutletContext } from "react-router-dom"
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
import { Subscription } from "../../lib/types"
import { addSubscription } from "../../lib/api"

const SubscriptionsPage = () => {
    const [showAddModal, setShowAddModal] = useState(false)
    const [addRssLink, setAddRssLink] = useState("")
    const { subsById } = useOutletContext()
    const [subs, setSubs] = useState<Subscription[]>(Object.values(subsById))
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
                            data={Object.values(subsById)}
                            cmpKeys={["title", "description"]}
                            onSearch={(filtered) => setSubs(filtered)}
                        />,
                    ]}
                />
                <div className={styles.subs}>
                    {subs.length === 0 ? (
                        <p>No channels found.</p>
                    ) : (
                        subs.map((sub) => (
                            <SubscriptionCard
                                sub={sub}
                                onDelete={() => {
                                    setSubs(subs.filter((s) => s.id !== sub.id))
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
                    onDone={async () => {
                        const sub = await addSubscription(addRssLink)
                        setSubs([...subs, sub])
                        console.log(sub)
                    }}
                />
            </Layout>
        </Layout>
    )
}

export default SubscriptionsPage
