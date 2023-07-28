import styles from "./style.module.scss"
import { FiMoreVertical } from "react-icons/fi"
import { MdDelete } from "react-icons/md"
import cx from "classnames"
import { useEffect, useRef, useState } from "react"
import { Link } from "react-router-dom"
import { Subscription } from "../../lib/types"
import { deleteSubscription } from "../../lib/api"

interface Props {
    sub: Subscription
    onDelete: () => void
}

const SubscriptionCard = ({ sub, onDelete }: Props) => {
    const { id, image, title, num_episodes } = sub
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
            <div
                className={cx(styles.card, {
                    [styles.unwrapped]: !openContext,
                })}
            >
                <img src={image} />
                <Link to={`/subscriptions/channel/${id}`}>
                    <h3>{title}</h3>
                </Link>
                <p>{num_episodes} episodes</p>

                <div
                    className={cx(styles.contextMenu, {
                        [styles.circle]: openContext,
                    })}
                    onMouseUp={() => {
                        console.log(openContext)
                        setOpenContext(!openContext)
                    }}
                >
                    <FiMoreVertical />
                </div>
            </div>
            <div
                className={cx(styles.menu, { [styles.show]: openContext })}
                tabIndex={0}
                ref={focusRef}
                onBlur={() => setOpenContext(false)}
            >
                <div
                    className={cx(styles.delete, styles.menuItem)}
                    onClick={async () => {
                        const res = await deleteSubscription(id)
                        onDelete()
                        console.log(res)
                    }}
                >
                    <MdDelete />
                    <span>Delete</span>
                </div>
            </div>
        </div>
    )
}

export default SubscriptionCard
