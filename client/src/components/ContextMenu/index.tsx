import { useEffect, useRef, useState } from "react"
import styles from "./style.module.scss"
import { FiMoreVertical } from "react-icons/fi"
import cx from "classnames"

const ContextMenu = () => {
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
                className={cx(styles.contextMenu, {
                    [styles.circle]: openContext,
                })}
                onMouseUp={() => {
                    setOpenContext(!openContext)
                }}
            >
                <FiMoreVertical />
            </div>
            <div
                className={cx(styles.menu, { [styles.show]: openContext })}
                tabIndex={0}
                ref={focusRef}
                onBlur={() => setOpenContext(false)}
            >
                <div className={styles.menuItem}>
                    <span>Delete</span>
                </div>
            </div>
        </div>
    )
}

export default ContextMenu
