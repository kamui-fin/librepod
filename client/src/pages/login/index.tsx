import Button from "@/components/Button"
import styles from "./style.module.scss"
import Input from "@/components/Input"

import { Formik, ErrorMessage } from "formik"
import { z } from "zod"
import { toFormikValidationSchema } from "zod-formik-adapter"

import { axios } from "@/lib/api"
import { useNavigate } from "react-router-dom"
import { useAuth } from "../../lib/useAuth"

const Schema = z.object({
    usernameOrEmail: z.string(),
    password: z.string(),
})

const LoginPage = () => {
    const navigate = useNavigate()
    const { login } = useAuth()
    return (
        <Formik
            initialValues={{ usernameOrEmail: "", password: "" }}
            validationSchema={toFormikValidationSchema(Schema)}
            onSubmit={(values, { setSubmitting }) => {
                login(values, () => setSubmitting(false))
            }}
        >
            {({
                values,
                errors,
                touched,
                handleChange,
                handleBlur,
                handleSubmit,
                isSubmitting,
            }) => (
                <form onSubmit={handleSubmit} className={styles.formContainer}>
                    <div className={styles.card}>
                        <div className={styles.starter}>
                            <h1>Welcome</h1>
                            <p>Sign in to access your podcast feed</p>
                        </div>
                        <ErrorMessage name="usernameOrEmail" component="div" />
                        <Input
                            type="text"
                            name="usernameOrEmail"
                            placeholder="Username or E-mail"
                            onChange={handleChange}
                            onBlur={handleBlur}
                            value={values.usernameOrEmail}
                        />
                        <ErrorMessage name="password" component="div" />
                        <Input
                            type="password"
                            name="password"
                            placeholder="Password"
                            onChange={handleChange}
                            onBlur={handleBlur}
                            value={values.password}
                        />
                        <Button submit disabled={isSubmitting}>
                            Sign In
                        </Button>
                        <div className={styles.redirect}>
                            <span>Don't have an account?</span>
                            <a href="/register">Register</a>
                        </div>
                    </div>
                </form>
            )}
        </Formik>
    )
}

export default LoginPage
