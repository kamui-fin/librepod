import { Link } from "react-router-dom"
import Button from "../../components/Button"
import Layout from "../../components/Layout"
import SearchBar from "../../components/Search"
import SubscriptionCard from "../../components/SubscriptionCard"
import styles from "./style.module.scss"
import { IoMdAdd } from "react-icons/io"
import ActionTitleBar from "../../components/ActionTitleBar"

const subs = [
    {
        image: "https://i.typlog.com/bitvoice/8446580582_247966.png",
        title: "比特新声",
        numEpisodes: 324,
    },
]

const SubscriptionsPage = () => {
    return (
        <Layout>
            <ActionTitleBar title="Subscriptions" actions={[
                <Button><IoMdAdd /></Button>,
                <SearchBar text="Search channels" />
            ]} />
            {subs.map((sub) => (
                <SubscriptionCard {...sub} />
            ))}
        </Layout>
    )
}

export default SubscriptionsPage
