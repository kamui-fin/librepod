import { createContext, useContext, useMemo } from "react"
import { useLocalStorage } from "@/lib/useLocalStorage"
import { axios, registerUser, loginUser } from "./api"
import { LoginBody } from "../pages/login"
import { RegisterBody } from "../pages/register"

export interface User {
    id: string
    name: string
    email: string
    password: string // TODO: remove
    salt: number[]
    created_at: string // convert to date
}

interface Auth {
    user: User | null
    register: (values: RegisterBody, onDone: () => void) => Promise<void>
    login: (values: LoginBody, onDone: () => void) => Promise<void>
    logout: () => Promise<void>
}

const AuthContext = createContext<Auth | null>(null)

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
    const [user, setUser] = useLocalStorage<User | null>("user", null)

    const register = async (
        values: RegisterBody,
        onDone: () => void
    ) => {
        const user = await registerUser(values);
        setUser(user)
        onDone()
    }

    const login = async (values: LoginBody, onDone: () => void) => {
        const user = await loginUser(values);
        setUser(user)
        onDone()
    }

    const logout = async () => {
        await logout()
        setUser(null)
    }

    const value = useMemo(
        () => ({
            user,
            register,
            login,
            logout,
        }),
        [user]
    )
    return (
        <AuthContext.Provider value={value}> {children}</AuthContext.Provider>
    )
}

export const useAuth = () => {
    return useContext(AuthContext)
}
