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
        {
            path: "/song-edit/:uuid/midi-editor",
            lazy: async () => {
                const { SongEditMidiEditorRoute } = await import("./routes/song-edit/midi-editor");
                return { Component: SongEditMidiEditorRoute }
            }
        },
        {
            path: "/song-edit/:uuid/details",
            lazy: async () => {
                const { SongEditSongDetailsRoute } = await import("./routes/song-edit/song-details");
                return { Component: SongEditSongDetailsRoute }
            }
        },
        {
            path: "/song-play/:uuid",
            lazy: async () => {
                const { SongPlayRoute } = await import("./routes/song-play");
                return { Component: SongPlayRoute }
            }
        },
    ])
}

export const AppRouter = () => {
    const router = useMemo(() => createAppRouter(), [])
    return <RouterProvider router={router} />
}
