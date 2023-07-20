import Button from "../../components/Button"
import Layout from "../../components/Layout"
import styles from "./style.module.scss"
import SearchBar from "../../components/Search"
import { BsFillPlayFill } from "react-icons/bs"
import EpisodeList from "../../components/EpisodeList"
import ActionTitleBar from "../../components/ActionTitleBar"
import ConfirmationModal from "../../components/Modal"
import { useLoaderData, useOutletContext } from "react-router-dom"
import { Episode } from "../../lib/types"

const FeedPage = () => {
    const { episodes, subsById } = useOutletContext()
    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar
                    title="Feed"
                    actions={[
                        <Button secondary>
                            <BsFillPlayFill />
                            <span>Play All</span>
                        </Button>,
                        <SearchBar text="Find episodes" />,
                    ]}
                />
                <EpisodeList items={episodes} channels={subsById} />
            </Layout>
            <ConfirmationModal></ConfirmationModal>
        </Layout>
    )
}

export default FeedPage
