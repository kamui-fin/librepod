import { createContext, useContext, useMemo } from "react"
import { useLocalStorage } from "@/lib/useLocalStorage"
import { axios } from "./api"
import { LoginBody } from "../pages/login"
import { RegisterBody } from "../pages/register"

interface User {
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
        const { data }: { data: User } = await axios.put(
            "/auth/register",
            values
        )
        setUser(data)
        onDone()
    }

    const login = async (values: LoginBody, onDone: () => void) => {
        const { data }: { data: User } = await axios.put("/auth/login", values)
        setUser(data)
        onDone()
    }

    const logout = async () => {
        try {
            await axios.put("/auth/logout")
        } catch (e) {
            console.log(e)
        }
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
