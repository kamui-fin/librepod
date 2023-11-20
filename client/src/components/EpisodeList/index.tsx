import { Episode } from "../../lib/types"
import { getHumanDate } from "../../lib/utils"
import EpisodeListItem from "./EpisodeListItem"
import styles from "./style.module.scss"

interface Props {
    items: Episode[]
    withoutDate?: boolean
    withThumbnail?: boolean
}

const EpisodeList = ({
    items,
    withoutDate = false,
    withThumbnail = false,
}: Props) => {
    const groupByDate = <T extends Episode>(eps: T[]) => {
        const grouped: { [key: string]: T[] } = {}
        for (const ep of eps) {
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
    } else if (!withoutDate) {
        return (
            <div className={styles.container}>
                {Object.entries(groupByDate(items)).map(([date, items]) => (
                    <div className={styles.dateGroup}>
                        <h2 className={styles.dateHeader}>{date}</h2>
                        <div className={styles.list}>
                            {items.map((item) => (
                                <EpisodeListItem
                                    episode={item}
                                    withThumbnail={withThumbnail}
                                />
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        )
    } else {
        return (
            <div className={styles.container}>
                <div className={styles.list}>
                    {items.map((item) => (
                        <EpisodeListItem
                            episode={item}
                            withThumbnail={withThumbnail}
                        />
                    ))}
                </div>
            </div>
        )
    }
}

export default EpisodeList
