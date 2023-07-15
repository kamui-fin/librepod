import styles from "./style.module.scss"
import { FiMoreVertical } from "react-icons/fi"
import { MdDelete } from "react-icons/md"
import cx from "classnames"
import { useEffect, useRef, useState } from "react"
import { Link } from "react-router-dom"

interface Props {
    image: string
    title: string
    numEpisodes: number
}

const SubscriptionCard = ({ image, title, numEpisodes }: Props) => {
    const [openContext, setOpenContext] = useState<boolean>(false)
    const focusRef = useRef<HTMLDivElement>(null)
    useEffect(() => {
        if (openContext) {
            focusRef.current.focus()
        } else {
            focusRef.current.blur()
        }
    }, [openContext])
    return (
        <div className={styles.container}>
            <div className={cx(styles.card, { [styles.unwrapped]: !openContext })}>
                <img src={image} />
                <Link to="/subscriptions/channel">
                    <h3>{title}</h3>
                </Link>
                <p>{numEpisodes} episodes</p>

                <div className={cx(styles.contextMenu, { [styles.circle]: openContext })} onMouseUp={() => {
                    console.log(openContext);
                    setOpenContext(!openContext)
                }}>
                    <FiMoreVertical />
                </div>
            </div>
            <div
                className={cx(styles.menu, { [styles.show]: openContext })}
                tabIndex={0}
                ref={focusRef}
                onBlur={() => setOpenContext(false)}
            >
                <div className={cx(styles.delete, styles.menuItem)}>
                    <MdDelete />
                    <span>Delete</span>
                </div>
            </div>
        </div>
    )
}

export default SubscriptionCard
