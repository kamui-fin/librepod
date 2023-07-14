import { Episode } from ".."
import styles from "./style.module.scss"
import { AiFillPlayCircle } from "react-icons/ai"

interface Props {
    item: Episode
    channelOnly: boolean
}

const EpisodeListItem = ({ item, channelOnly }: Props) => {
    return (
        <div className={styles.listItem}>
            {
                channelOnly ? (
                    <div className={styles.meta}>
                        <span className={styles.channel}>{item.date.toLocaleDateString()}</span>
                        <h3 className={styles.title}>{item.title}</h3>
                        <p className={styles.desc}>{item.description}</p>
                    </div>
                ) : (
                    <>
                        <div className={styles.image}>
                            <img src={item.img} />
                        </div>
                        <div className={styles.meta}>
                            <h3 className={styles.title}>{item.title}</h3>
                            <p className={styles.desc}>{item.description}</p>
                            <span className={styles.channel}>{item.channelName}</span>
                        </div>
                    </>
                )
            }
            <div className={styles.play}>
                <button>
                    <AiFillPlayCircle />
                    <span>{item.duration}</span>
                </button>
            </div>
        </div>
    )
}

export default EpisodeListItem
