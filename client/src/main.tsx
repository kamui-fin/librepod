import React from "react"
import ReactDOM from "react-dom/client"
import "normalize.css"
import "./index.scss"
import { createBrowserRouter, RouterProvider } from "react-router-dom"
import RegisterPage from "./pages/register"
import LoginPage from "./pages/login"
import { ProtectedRoute, UnprotectedRoute } from "./components/AuthRoutes"
import Layout from "./components/Layout"
import HomePage from "./pages/home"
import FeedPage from "./pages/feed"
import ChannelsPage from "./pages/channels"
import SettingsPage from "./pages/settings"
import HistoryPage from "./pages/history"


const rootPaths = [
    {
        element: (
            <FeedPage />
        ),
        index: true
    },
    {
        path: "/subscriptions",
        element: (
            <ChannelsPage />
        )
    },
    {
        path: "/history",
        element: (
            <HistoryPage />
        )
    },
    {
        path: "/settings",
        element: (
            <SettingsPage />
        )
    }
]

const router = createBrowserRouter([
    {
        path: "/",
        element: (
            <ProtectedRoute>
                <Layout>
                    <HomePage />
                </Layout>
            </ProtectedRoute>
        ),
        children: rootPaths,
    },
    {
        path: "/register",
        element: (
            <UnprotectedRoute>
                <Layout>
                    <RegisterPage />
                </Layout >
            </UnprotectedRoute>
        ),
    },
    {
        path: "/login",
        element: (
            <UnprotectedRoute>
                <Layout>
                    <LoginPage />
                </Layout>
            </UnprotectedRoute>
        ),
    },
])

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <RouterProvider router={router} />
    </React.StrictMode>
)
