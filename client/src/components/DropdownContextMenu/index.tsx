import { ReactNode, useEffect, useRef, useState } from "react"
import styles from "./style.module.scss"
import cx from "classnames"
import { FiMoreVertical } from "react-icons/fi"
import { useClickOutside } from "@/lib/useClickOutside"

interface MenuItemProps {
    onClick: () => void
    setOpenContext: React.Dispatch<React.SetStateAction<boolean>>
    icon: ReactNode
    text: string
    className?: string
}

export const MenuItem = ({
    icon,
    text,
    onClick,
    setOpenContext,
    className,
}: MenuItemProps) => {
    return (
        <div
            className={cx(styles.delete, styles.menuItem, className)}
            onMouseDown={(e) => {
                e.preventDefault()
            }}
            onClick={() => {
                onClick()
                setOpenContext(false)
            }}
        >
            {icon}
            <span>{text}</span>
        </div>
    )
}

interface Props {
    onChange: (open: boolean) => void
    menuItemProps: Omit<MenuItemProps, "setOpenContext">[]
}

const DropdownContextMenu = ({ onChange, menuItemProps }: Props) => {
    const toggleButtonRef = useRef(null)
    const contextMenuRef = useRef(null)
    const [openContext, setOpenContext] = useState<boolean>(false)

    useEffect(() => {
        onChange(openContext)
    }, [toggleButtonRef, contextMenuRef, onChange, openContext])

    useClickOutside([toggleButtonRef, contextMenuRef], () => {
        setOpenContext(false)
    })

    return (
        <>
            <div
                ref={toggleButtonRef}
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
                ref={contextMenuRef}
                className={cx(styles.menu, { [styles.show]: openContext })}
                tabIndex={0}
            >
                {menuItemProps.map((props) => (
                    <MenuItem {...props} setOpenContext={setOpenContext} />
                ))}
            </div>
        </>
    )
}

export default DropdownContextMenu
