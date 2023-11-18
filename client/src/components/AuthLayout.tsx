import { AuthProvider } from "@/lib/useAuth"
import { ReactNode } from "react"

interface Props {
    children: ReactNode
}

const AuthLayout = ({ children }: Props) => {
    return <AuthProvider>{children}</AuthProvider>
}

export default AuthLayout
