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
import { feedLoader, getFeed, getSubscription, getSubscriptions } from "./lib/api"

const rootPaths = [
    {
        element: <FeedPage />,
        index: true,
    },
    {
        path: "/subscriptions/channel/:name",
        element: <ChannelPage />,
        loader: getSubscription,
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
            <AuthLayout>
                <ProtectedRoute>
                    <HomePage />
                </ProtectedRoute>
            </AuthLayout>
        ),
        children: rootPaths,
        loader: feedLoader
    },
    {
        path: "/register",
        element: (
            <AuthLayout>
                <UnprotectedRoute>
                    <RegisterPage />
                </UnprotectedRoute>
            </AuthLayout>
        ),
    },
    {
        path: "/login",
        element: (
            <AuthLayout>
                <UnprotectedRoute>
                    <LoginPage />
                </UnprotectedRoute>
            </AuthLayout>
        ),
    },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
)
