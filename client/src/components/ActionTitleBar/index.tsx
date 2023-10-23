import { useNavigate } from "react-router-dom"
import Divider from "../Divider"
import styles from "./style.module.scss"
import { BiArrowBack } from "react-icons/bi"

// If no title is provided, back button will be used in place
interface Props {
    title?: string
    actions?: React.ReactNode[]
}

const ActionTitleBar = ({ title, actions }: Props) => {
    const navigate = useNavigate()
    const goBack = () => {
        navigate(-1)
    }
    return (
        <>
            <header>
                {title ? (
                    <h1>{title}</h1>
                ) : (
                    <div className={styles.back} onClick={goBack}>
                        <BiArrowBack />
                        <span>Back</span>
                    </div>
                )}
                <div className={styles.actions}>{actions}</div>
            </header>
            <Divider />
        </>
    )
}

export default ActionTitleBar
