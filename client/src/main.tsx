import React from "react"
import ReactDOM from "react-dom/client"
import "normalize.css"
import "./index.scss"
import { createBrowserRouter, RouterProvider } from "react-router-dom"
import RegisterPage from "./pages/register"
import LoginPage from "./pages/login"
import { ProtectedRoute, UnprotectedRoute } from "./components/AuthRoutes"
import Layout from "./components/Input/Layout"

const router = createBrowserRouter([
    {
        path: "/",
        element: (
            <Layout>
                <ProtectedRoute>
                    <div>You are logged in</div>
                </ProtectedRoute>
            </Layout>
        ),
    },
    {
        path: "/register",
        element: (
            <Layout>
                <UnprotectedRoute>
                    <RegisterPage />
                </UnprotectedRoute>
            </Layout>
        ),
    },
    {
        path: "/login",
        element: (
            <Layout>
                <UnprotectedRoute>
                    <LoginPage />
                </UnprotectedRoute>
            </Layout>
        ),
    },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
)
