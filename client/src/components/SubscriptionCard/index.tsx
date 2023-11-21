import styles from "./style.module.scss"
import cx from "classnames"
import { Link } from "react-router-dom"
import { Channel } from "../../lib/types"
import DropdownContextMenu from "../DropdownContextMenu"
import { useState } from "react"
import { MdDelete } from "react-icons/md"

interface Props {
    sub: Channel
    onDelete: () => void
}

const SubscriptionCard = ({ sub, onDelete }: Props) => {
    const { id, image, title, num_episodes } = sub
    const [contextOpen, setContextOpen] = useState(false)
    return (
        <div className={styles.container}>
            <div
                className={cx(styles.card, {
                    [styles.unwrapped]: !contextOpen,
                })}
            >
                <img src={image || ""} />
                <Link to={`/subscriptions/channel/${id}`}>
                    <h3>{title}</h3>
                </Link>
                <p>{num_episodes} episodes</p>
                <DropdownContextMenu
                    className={styles.dropdown}
                    onChange={setContextOpen}
                    menuItemProps={[
                        {
                            icon: <MdDelete />,
                            text: "Delete",
                            onClick: onDelete,
                            className: styles.delete,
                        },
                    ]}
                />
            </div>
        </div>
    )
}

export default SubscriptionCard
