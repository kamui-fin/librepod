import { ScaleLoader } from "react-spinners"
import styles from "./style.module.scss"

interface Props {
    isLoading: boolean
    children: React.ReactNode
}

const Loader = ({ isLoading, children }: Props) => {
    return isLoading ? (
        <div className={styles.loader}>
            <ScaleLoader color="#25ae64" width={5} height={50} />
        </div>
    ) : (
        <>{children}</>
    )
}

export default Loader
