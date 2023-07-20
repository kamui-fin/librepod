import { Episode } from "../../lib/types"
import EpisodeListItem from "./EpisodeListItem"
import styles from "./style.module.scss"

interface Props {
    items: Episode[]
    channelOnly?: boolean
}

const EpisodeList = ({ items, channelOnly = false }: Props) => {
    const groupByDate = (eps: Episode[]) => {
        let grouped: { [key: string]: Episode[] } = {}
        for (let ep of eps) {
            const date = new Date(ep.published / 1000).toLocaleDateString()
            if (date in grouped) {
                grouped[date].push(ep)
            } else {
                grouped[date] = new Array(ep)
            }
        }
        return grouped
    }
    return (
        <div className={styles.container}>
            {channelOnly ? (
                <div className={styles.list}>
                    {items.map((item) => (
                        <EpisodeListItem channelOnly item={item} />
                    ))}
                </div>
            ) : (
                Object.entries(groupByDate(items)).map(([date, items]) => (
                    <div className={styles.dateGroup}>
                        <h2 className={styles.dateHeader}>{date}</h2>
                        <div className={styles.list}>
                            {items.map((item) => (
                                <EpisodeListItem item={item} />
                            ))}
                        </div>
                    </div>
                ))
            )}
        </div>
    )
}

export default EpisodeList
