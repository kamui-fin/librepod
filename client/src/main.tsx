import React from "react"
import ReactDOM from "react-dom/client"
import "normalize.css"
import "./index.scss"
import { createBrowserRouter, RouterProvider } from "react-router-dom"
import RegisterPage from "./pages/register"
import LoginPage from "./pages/login"
import { ProtectedRoute, UnprotectedRoute } from "./components/AuthRoutes"
import AuthLayout from "./components/AuthLayout"
import HomePage from "./pages/home"
import FeedPage from "./pages/feed"
import SubscriptionsPage from "./pages/subscriptions"
import SettingsPage from "./pages/settings"
import HistoryPage from "./pages/history"
import ChannelPage from "./pages/channel"

const rootPaths = [
    {
        element: <FeedPage />,
        index: true,
    },
    {
        path: "/subscriptions/channel/:name",
        element: <ChannelPage />,
    },
    {
        path: "/subscriptions",
        element: <SubscriptionsPage />,
    },
    {
        path: "/history",
        element: <HistoryPage />,
    },
    {
        path: "/settings",
        element: <SettingsPage />,
    },
]

const router = createBrowserRouter([
    {
        path: "/",
        element: (
            <ProtectedRoute>
                <AuthLayout>
                    <HomePage />
                </AuthLayout>
            </ProtectedRoute>
        ),
        children: rootPaths,
    },
    {
        path: "/register",
        element: (
            <UnprotectedRoute>
                <AuthLayout>
                    <RegisterPage />
                </AuthLayout>
            </UnprotectedRoute>
        ),
    },
    {
        path: "/login",
        element: (
            <UnprotectedRoute>
                <AuthLayout>
                    <LoginPage />
                </AuthLayout>
            </UnprotectedRoute>
        ),
    },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
)
