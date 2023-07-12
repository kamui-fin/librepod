import { AuthProvider } from "@/lib/useAuth"

const AuthLayout = ({ children }) => {
    return <AuthProvider>{children}</AuthProvider>
}

export default AuthLayout
