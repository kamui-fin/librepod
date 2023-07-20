import { createContext, useContext, useEffect, useMemo } from "react"
import { useNavigate } from "react-router-dom"
import { useLocalStorage } from "@/lib/useLocalStorage"
import { axios } from "./api"

const AuthContext = createContext({})

export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
    const [user, setUser] = useLocalStorage("user", null)
    const navigate = useNavigate()

    const register = async (values, onDone: () => void) => {
        const { data } = await axios.put("/auth/register", values)
        setUser(data, onDone)
    }

    const login = async (values, onDone: () => void) => {
        const { data } = await axios.put("/auth/login", values)
        setUser(data, onDone)
    }

    const logout = async () => {
        try {
            await axios.put("/auth/logout")
        } catch { }
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
