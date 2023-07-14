import styles from "./style.module.scss"
import { FiMoreVertical } from "react-icons/fi"
import { MdDelete } from "react-icons/md"
import cx from "classnames"
import { useState } from "react"
import { Link } from "react-router-dom"

interface Props {
    image: string
    title: string
    numEpisodes: number
}

const SubscriptionCard = ({ image, title, numEpisodes }: Props) => {
    const [openContext, setOpenContext] = useState<boolean>(false)
    return (
        <div className={styles.card}>
            <img src={image} />
            <Link to="/channel">
            <h3>{title}</h3>
                </Link>
            <p>{numEpisodes} episodes</p>

            <div className={styles.contextMenu}>
                <div onClick={() => setOpenContext(!openContext)}>
                    <FiMoreVertical />
                </div>
                <div
                    className={cx(styles.menu, { [styles.show]: openContext })}
                >
                    <div className={cx(styles.delete, styles.menuItem)}>
                        <MdDelete />
                        <span>Delete</span>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default SubscriptionCard