import { useEffect, useRef, useState } from "react"
import styles from "./style.module.scss"
import { FiMoreVertical } from "react-icons/fi"
import cx from "classnames"

export interface MenuItem {
    text: string
    handler: () => void
}

const ContextMenu = ({ menuItems }: { menuItems: MenuItem[] }) => {
    const [openContext, setOpenContext] = useState<boolean>(false)
    const focusRef = useRef<HTMLDivElement>(null)
    useEffect(() => {
        if (openContext) {
            focusRef?.current?.focus()
        } else {
            focusRef?.current?.blur()
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
                {menuItems.map((item) => (
                    <div className={styles.menuItem} onClick={item.handler}>
                        <span>{item.text}</span>
                    </div>
                ))}
            </div>
        </div>
    )
}

export default ContextMenu
