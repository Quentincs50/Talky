import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { createBrowserRouter, Navigate, RouterProvider } from 'react-router-dom'
import App from './App'
import AuthProvider from './components/auth/AuthProvider'
import ProtectedRoute from './components/auth/ProtectedRoute'
import './index.css'
import ChannelPage from './pages/private/ChannelPage'
import MessagePage from './pages/private/MessagePage'
import ProfilePage from './pages/private/ProfilePage'
import ServerPage from './pages/private/ServerPage'
import AuthenticationPage from './pages/public/AuthenticationPage'
import ServerDetailsPage from './pages/private/ServerDetailsPage'
import InvitePage from './pages/private/InvitePage'
import JoinServerPage from './pages/private/JoinServerPage'

const router = createBrowserRouter([
    {
        path: "/",
        element: <App />,
        children: [
            {
                path: "/",
                element: <Navigate to="/servers" />,
            },
            {
                path: "/authentication",
                element:
                    <AuthenticationPage />,
            },
            {
                path: "servers",
                element: (
                    <ProtectedRoute>
                        <ServerPage />
                    </ProtectedRoute>
                ),
            },
            {
                path: "/servers/:serverId/channel",
                element: (
                    <ProtectedRoute>
                        <ChannelPage />
                    </ProtectedRoute>
                ),
            },
            {
                path: "/invite/:code",
                element: (
                    <ProtectedRoute>
                        <InvitePage />
                    </ProtectedRoute>
                ),
            },
            {
                path: "/join",
                element: (
                    <ProtectedRoute>
                        <JoinServerPage />
                    </ProtectedRoute>
                ),
            },
            {
                path: "/channel/:id/messages",
                element: (
                    <ProtectedRoute>
                        <MessagePage />
                    </ProtectedRoute>
                ),
            },
            {
                path: "/servers/:serverId",
                element: (
                    <ProtectedRoute>
                        <ServerDetailsPage />
                    </ProtectedRoute>
                ),
            },
            {
                path: "profile/:id",
                element: (
                    <ProtectedRoute>
                        <ProfilePage />
                    </ProtectedRoute>
                ),
            },
        ]
    }
])

createRoot(document.getElementById("root")).render(
    <StrictMode>
        <AuthProvider>
            <RouterProvider router={router} />
        </AuthProvider>
    </StrictMode>,
);
