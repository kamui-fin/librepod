import { AuthProvider } from "@/lib/useAuth"

const Layout = ({ children }) => {
    return (<AuthProvider>
        {children}
    </AuthProvider>)
}

export default Layout
