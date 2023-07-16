import { SetStateAction, useState } from "react"
import Button from "../Button"
import Divider from "../Divider"
import styles from "./style.module.scss"
import cx from "classnames"

interface Props {
    content: React.ReactNode
    primary: boolean
    title: string
    actionName: string

    open: boolean
    setOpen: React.Dispatch<SetStateAction<boolean>>
}

const Modal = ({
    actionName,
    content,
    primary,
    title,
    open,
    setOpen,
}: Props) => {
    return (
        <>
            <div className={cx(styles.modal, { [styles.closed]: !open })}>
                <h2 className={styles.title}>{title}</h2>
                <div className={styles.content}>{content}</div>
                <Divider />
                <div className={styles.actions}>
                    <Button
                        className={styles.cancel}
                        onClick={() => setOpen(false)}
                    >
                        Cancel
                    </Button>
                    <Button secondary={!primary}>{actionName}</Button>
                </div>
            </div>
            <div
                className={cx(styles.pageMask, { [styles.closed]: !open })}
            ></div>
        </>
    )
}

export default Modal
