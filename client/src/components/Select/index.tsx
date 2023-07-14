import cx from "classnames"
import styles from "./style.module.scss"

interface Props {
    icon?: React.ReactNode
    onDone?: (text: string) => void
    items: string[]
    defaultIndex?: number
}

const Select = ({ icon, onDone, items, defaultIndex = 0 }: Props) => {
    return (
        <div className={styles.container}>
            {icon && <div className={styles.iconBox}>{icon}</div>}
            <select
                className={cx(styles.select, { [styles.withIcon]: !!icon })}
                onChange={(ev) => {
                    onDone && onDone(items[ev.target.value])
                }}
            >
                {items.map((item, indx) => (
                    <option className={styles.item} value={indx} selected={defaultIndex === indx}>
                        {item}
                    </option>
                ))}
            </select>
        </div>
    )
}

export default Select
