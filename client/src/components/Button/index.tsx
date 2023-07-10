import styles from "./style.module.scss"

interface Props {
    submit?: boolean
    children: React.ReactElement | string
    disabled: boolean
}

const Button = ({ submit, children, disabled }: Props) => {
    if (submit) {
        return (
            <input
                className={styles.btn}
                disabled={disabled}
                type="submit"
                value={children.toString()}
            />
        )
    } else {
        return <button className={styles.btn} disabled={disabled} >{children}</button>
    }
}

export default Button
