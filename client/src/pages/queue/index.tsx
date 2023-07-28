import React from "react"
import Layout from "../../components/Layout"
import SearchBar from "../../components/Search"
import ActionTitleBar from "../../components/ActionTitleBar"

const QueuePage = () => {
    return (
        <Layout>
            <Layout inner>
                <ActionTitleBar title="Queue" />
            </Layout>
        </Layout>
    )
}

export default QueuePage
