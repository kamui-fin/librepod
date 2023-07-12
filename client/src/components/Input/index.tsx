import { InputHTMLAttributes } from "react"
import styles from "./style.module.scss"
import React from "react"
import cx from "classnames"

const Input = ({className, ...props}: InputHTMLAttributes<HTMLInputElement>) => {
    return <input className={cx(styles.input, className)} {...props} />
}

export default Input
