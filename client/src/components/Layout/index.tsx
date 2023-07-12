import styles from "./index.module.scss"

const Layout = ({ children }) => {
    return <main className={styles.container}>{children}</main>
}

export default Layout
