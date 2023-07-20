import { ToastContainer } from "react-toastify"
import styles from "./index.module.scss"
import cx from "classnames"

interface Props {
    inner?: boolean
    children: React.ReactNode
}

const Layout = ({ children, inner = false }: Props) => {
    return (
        <main className={cx(styles.container, { [styles.inner]: inner })}>
            {children}
            <ToastContainer
                position="top-center"
                autoClose={false}
                hideProgressBar={false}
                newestOnTop={false}
                closeOnClick
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
                theme="dark"
                toastStyle={{ backgroundColor: "#2d343f" }}
            />
        </main>
    )
}

export default Layout
