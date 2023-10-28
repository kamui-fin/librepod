import Button from "@/components/Button"
import styles from "./style.module.scss"
import Input from "@/components/Input"

import { Formik, ErrorMessage } from "formik"
import { z } from "zod"
import { toFormikValidationSchema } from "zod-formik-adapter"
import { useNavigate } from "react-router-dom"
import { useAuth } from "@/lib/useAuth"
import { toast } from "react-toastify"

export interface RegisterBody {
    username: string;
    email: string;
    password: string;
    confirmPassword: string;
}

const RegisterSchema = z
    .object({
        username: z
            .string()
            .min(3)
            .max(20)
            .regex(/^[a-zA-Z0-9_-]*$/, {
                message:
                    "Must be alphanumeric. Only '-' and '_' symbols are allowed",
            }),
        email: z.string().email(),
        password: z.string().min(6),
        confirmPassword: z.string().min(6),
    })
    .superRefine(({ confirmPassword, password }, ctx) => {
        if (confirmPassword !== password) {
            ctx.addIssue({
                path: ["confirmPassword"],
                message: "The passwords did not match",
                code: "custom",
            })
        }
    })

const RegisterPage = () => {
    const navigate = useNavigate()
    const { register } = useAuth() ?? {}
    if (!register) return <></>
    return (
        <Formik
            initialValues={{
                username: "",
                email: "",
                password: "",
                confirmPassword: "",
            }}
            validationSchema={toFormikValidationSchema(Schema)}
            onSubmit={async (values, { setSubmitting }) => {
                try {
                    await register(values, () => setSubmitting(false))
                } catch (err) {
                    toast.error("Failed to register", {
                        position: "top-center",
                        autoClose: 5000,
                    })
                }
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
                            <h1>Sign Up</h1>
                            <p>Let's setup your new account</p>
                        </div>
                        <ErrorMessage name="name" component="div" />
                        <Input
                            type="text"
                            name="username"
                            placeholder="Name"
                            onChange={handleChange}
                            onBlur={handleBlur}
                            value={values.username}
                        />
                        <ErrorMessage name="email" component="div" />
                        <Input
                            type="text"
                            name="email"
                            placeholder="Email Address"
                            onChange={handleChange}
                            onBlur={handleBlur}
                            value={values.email}
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
                        <ErrorMessage name="confirmPassword" component="div" />
                        <Input
                            type="password"
                            name="confirmPassword"
                            placeholder="Confirm Password"
                            onBlur={handleBlur}
                            onChange={handleChange}
                            value={values.confirmPassword}
                        />
                        <Button submit disabled={isSubmitting}>
                            Sign In
                        </Button>
                        <div className={styles.redirect}>
                            <span>Already have an account?</span>
                            <a href="/login">Log In</a>
                        </div>
                    </div>
                </form>
            )}
        </Formik>
    )
}

export default RegisterPage
