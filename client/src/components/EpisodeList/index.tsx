import { Episode, Subscription } from "../../lib/types"
import { getHumanDate } from "../../lib/utils"
import EpisodeListItem from "./EpisodeListItem"
import styles from "./style.module.scss"

interface Props {
    items: Episode[]
    channels: { [key: string]: Subscription }
    channelOnly?: boolean
    withoutDate?: boolean
}

const EpisodeList = ({
    items,
    channels,
    channelOnly = false,
    withoutDate = false,
}: Props) => {
    const groupByDate = (eps: Episode[]) => {
        let grouped: { [key: string]: Episode[] } = {}
        for (let ep of eps) {
            const date = getHumanDate(ep.published)
            if (date in grouped) {
                grouped[date].push(ep)
            } else {
                grouped[date] = new Array(ep)
            }
        }
        return grouped
    }

    if (items.length == 0) {
        return (
            <div className={styles.container}>
                <p>No episodes were found.</p>
            </div>
        )
    } else if (withoutDate || channelOnly) {
        return (
            <div className={styles.container}>
                <div className={styles.list}>
                    {items.map((item) => (
                        <EpisodeListItem
                            channelOnly={channelOnly}
                            channel={channels[item.channel_id]}
                            item={item}
                        />
                    ))}
                </div>
            </div>
        )
    } else {
        return (
            <div className={styles.container}>
                {Object.entries(groupByDate(items)).map(([date, items]) => (
                    <div className={styles.dateGroup}>
                        <h2 className={styles.dateHeader}>{date}</h2>
                        <div className={styles.list}>
                            {items.map((item) => (
                                <EpisodeListItem
                                    item={item}
                                    channelOnly={channelOnly}
                                    channel={channels[item.channel_id]}
                                />
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        )
    }
}

export default EpisodeList
