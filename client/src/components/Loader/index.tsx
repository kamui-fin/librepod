import { ScaleLoader } from "react-spinners"
import styles from "./style.module.scss"
import cx from "classnames"
import { forwardRef } from "react"

interface Props {
    isLoading: boolean
    children?: React.ReactNode
    className?: string
}

const Loader = forwardRef<HTMLDivElement, Props>(
    ({ isLoading, children, className }: Props, ref) => {
        return isLoading ? (
            <div
                ref={ref}
                className={cx(className, styles.loader, {
                    [styles.full]: children !== undefined,
                })}
            >
                <ScaleLoader color="#25ae64" width={5} height={50} />
            </div>
        ) : (
            <>{children}</>
        )
    },
)

export default Loader
