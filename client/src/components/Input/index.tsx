import { InputHTMLAttributes } from "react"
import styles from "./style.module.scss"
import React from "react"

const Input = (props: InputHTMLAttributes<HTMLInputElement>) => {
    return (
        <input
            className={styles.input}
            {...props}
        />
    )
}

export default Input
