import { useMemo } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom"

export const createAppRouter = () => {
    return createBrowserRouter([
        {
            path: "/",
            lazy: async () => {
                const { IndexRoute } = await import("./routes/index");
                return { Component: IndexRoute }
            }
        }
    ])
}

export const AppRouter = () => {
    const router = useMemo(() => createAppRouter(), [])
    return <RouterProvider router={router} />
}
