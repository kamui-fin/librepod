import { Navigate } from "react-router-dom"
import { useAuth } from "@/lib/useAuth"
import { ReactNode } from "react"

interface ChildrenProps {
    children: ReactNode;
}

export const ProtectedRoute = ({ children }: ChildrenProps) => {
    const  auth  = useAuth()
    if (!auth || (auth && !auth.user)) 
        return <Navigate to="/login" />
    return children
}

export const UnprotectedRoute = ({ children }: ChildrenProps) => {
    const auth = useAuth()
    if (auth && auth.user)
        return <Navigate to="/" />
    return children
}
