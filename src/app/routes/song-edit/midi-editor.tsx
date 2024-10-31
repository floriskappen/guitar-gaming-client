import { Sidebar } from "@/features/song-edit/components/sidebar"
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { useParams } from "react-router-dom"
import { Menubar } from "@/features/song-edit/components/menubar"

export const SongEditMidiEditorRoute = () => {
    let { uuid } = useParams()

    return (
        <div className="flex">
            <SidebarProvider>
                <Sidebar />

                <div className="w-screen h-screen bg-neutral-900 pr-10 pt-2 pb-8">
                    <div className="flex pl-2">
                        <SidebarTrigger className="mt-1" />
                        <Menubar />

                    </div>
                    <div className="mt-4">
                        <div className="w-full px-8 ml-6">
                            <p>song edit - midi editor: {uuid}</p>
                        </div>
                    </div>
                </div>
            </SidebarProvider>
            
        </div>
    )
}
