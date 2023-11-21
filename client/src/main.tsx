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
import EpisodePage from "./pages/episode"
import QueuePage from "./pages/queue"
import {
    MutationCache,
    QueryCache,
    QueryClient,
    QueryClientProvider,
} from "@tanstack/react-query"
import { ReactQueryDevtools } from "@tanstack/react-query-devtools"
import { toast } from "react-toastify"

const rootPaths = [
    {
        element: <FeedPage />,
        index: true,
    },
    {
        path: "/subscriptions/channel/:id",
        element: <ChannelPage />,
    },
    {
        path: "/episode/:id",
        element: <EpisodePage />,
    },
    {
        path: "/subscriptions",
        element: <SubscriptionsPage />,
    },
    {
        path: "/queue",
        element: <QueuePage />,
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

const queryClient = new QueryClient({
    mutationCache: new MutationCache({
        onError: (error, query) => {
            toast.error(`Something went wrong: ${error.message}`, {
                position: "top-center",
                autoClose: 5000,
            })
        },
    }),
    queryCache: new QueryCache({
        onError: (error, query) => {
            toast.error(`Something went wrong: ${error.message}`, {
                position: "top-center",
                autoClose: 5000,
            })
        },
    }),
})

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <QueryClientProvider client={queryClient}>
            <RouterProvider router={router} />
            <ReactQueryDevtools initialIsOpen={false} />
        </QueryClientProvider>
    </React.StrictMode>,
)
