import styles from "./index.module.scss"
import cx from "classnames"

interface Props {
    inner?: boolean
    children: React.ReactNode;
}

const Layout = ({ children, inner = false }: Props) => {
    return <main className={cx(styles.container, { [styles.inner]: inner })}>{children}</main>
}

export default Layout
