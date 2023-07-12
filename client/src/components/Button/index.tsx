import styles from "./style.module.scss"
import cx from "classnames"

interface Props {
    submit?: boolean
    secondary: boolean
    children: React.ReactElement | string
    disabled?: boolean
}

const Button = ({ secondary, submit, children, disabled }: Props) => {
    if (submit) {
        return (
            <input
                className={cx(styles.btn, { [styles.secondary]: secondary })}
                disabled={disabled}
                type="submit"
                value={children.toString()}
            />
        )
    } else {
        return (
            <button
                className={cx(styles.btn, { [styles.secondary]: secondary })}
                disabled={disabled}
            >
                {children}
            </button>
        )
    }
}

export default Button
