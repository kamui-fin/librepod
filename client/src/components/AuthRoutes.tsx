import { Navigate } from "react-router-dom"
import { useAuth } from "@/lib/useAuth"

export const ProtectedRoute = ({ children }) => {
    const { user } = useAuth()
    if (user === null) {
        return <Navigate to="/login" />
    }
    return children
}

export const UnprotectedRoute = ({ children }) => {
    const { user } = useAuth()
    if (user) {
        return <Navigate to="/" />
    }
    return children
}
