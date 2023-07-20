import styles from "./style.module.scss"

import { BsFillPlayFill } from "react-icons/bs"
import EpisodeList, { Episode } from "../../components/EpisodeList"
import Button from "../../components/Button"
import SearchBar from "../../components/Search"
import Layout from "../../components/Layout"
import Select from "../../components/Select"
import cx from "classnames"
import { MdSort } from "react-icons/md"
import ActionTitleBar from "../../components/ActionTitleBar"
import Divider from "../../components/Divider"
import ChannelMeta, { Channel } from "../../components/ChannelMeta"
import { Subscription } from "../../lib/types"
import { useLoaderData } from "react-router-dom"

const ChannelPage = () => {
    const channel: Subscription = useLoaderData()
    return (
        <Layout>
            <ActionTitleBar actions={[<SearchBar text="Find episodes" />]} />
            <Layout inner>
                <ChannelMeta channel={channel} />
                <p className={styles.epCount}>
                    {channel.num_episodes} EPISODES
                </p>
                <Divider />
                <div className={cx(styles.actions, styles.spaceBottom)}>
                    <Button secondary>
                        <BsFillPlayFill />
                        <span>Play All</span>
                    </Button>
                    <Select
                        items={["Most Recent", "Least Recent"]}
                        icon={<MdSort />}
                    />
                </div>
                {/* <EpisodeList items={channel.episodes} channelOnly /> */}
            </Layout>
        </Layout>
    )
}

export default ChannelPage
