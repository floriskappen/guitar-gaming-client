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
        },
        {
            path: "/song-select",
            lazy: async () => {
                const { SongSelectRoute } = await import("./routes/song-select");
                return { Component: SongSelectRoute }
            }
        },
        {
            path: "/input-device-select",
            lazy: async () => {
                const { InputDeviceSelectRoute } = await import("./routes/input-device-select");
                return { Component: InputDeviceSelectRoute }
            }
        },
    ])
}

export const AppRouter = () => {
    const router = useMemo(() => createAppRouter(), [])
    return <RouterProvider router={router} />
}
