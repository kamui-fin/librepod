import styles from "./style.module.scss"
import cx from "classnames"

interface Props {
    submit?: boolean
    secondary?: boolean
    children: React.ReactElement | string
    disabled?: boolean
    className?: string
    onClick: () => void
}

const Button = ({
    className,
    secondary,
    submit,
    children,
    disabled,
    onClick,
}: Props) => {
    if (submit) {
        return (
            <input
                className={cx(styles.btn, className, {
                    [styles.secondary]: secondary,
                })}
                disabled={disabled}
                type="submit"
                value={children.toString()}
            />
        )
    } else {
        return (
            <button
                className={cx(styles.btn, className, {
                    [styles.secondary]: secondary,
                })}
                disabled={disabled}
                onClick={onClick}
            >
                {children}
            </button>
        )
    }
}

export default Button
