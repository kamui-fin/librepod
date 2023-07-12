import styles from "./style.module.scss"
import { FiMoreVertical } from "react-icons/fi"

interface Props {
    image: string
    title: string
    numEpisodes: number
}

const SubscriptionCard = ({ image, title, numEpisodes }: Props) => {
    return (
        <div className={styles.card}>
            <img src={image} />
            <h3>{title}</h3>
            <p>{numEpisodes} episodes</p>

            <div className={styles.contextMenu}>
                <FiMoreVertical />
            </div>
        </div>
    )
}

export default SubscriptionCard
