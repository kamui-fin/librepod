import { Link } from "react-router-dom"
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

const subs = [
    {
        image: "https://i.typlog.com/bitvoice/8446580582_247966.png",
        title: "比特新声",
        numEpisodes: 324,
    },
]

const SubscriptionsPage = () => {
    const [showAddModal, setShowAddModal] = useState(false)
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
            {subs.map((sub) => (
                <SubscriptionCard {...sub} />
            ))}
                <Modal
                    title="Add Subscription"
                    content={<Input placeholder="RSS Link" />}
                    actionName="Add"
                    primary={false}
                    open={showAddModal}
                    setOpen={setShowAddModal}
                />
        </Layout>
    )
}

export default SubscriptionsPage
