import { Link, useLoaderData } from "react-router-dom"
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
    const [subs, setSubs] = useState<Subscription[]>(useLoaderData())
    return (
        <Layout>
            <ActionTitleBar
                title="Subscriptions"
                actions={[
                    <Button onClick={() => setShowAddModal(true)}>
                        <IoMdAdd />
                    </Button>,
                    <SearchBar text="Search channels" />,
                ]}
            />
            <div className={styles.subs}>
                {subs.map((sub) => (
                    <SubscriptionCard
                        sub={sub}
                        onDelete={() => {
                            setSubs(subs.filter((s) => s.id !== sub.id))
                        }}
                    />
                ))}
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
    )
}

export default SubscriptionsPage
