import { SongEditSidebar } from "@/app/components/song-edit-sidebar"
import { Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut, MenubarTrigger } from "@/components/ui/menubar"
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { useParams } from "react-router-dom"

export const SongEditMapDetailsRoute = () => {
    let { uuid } = useParams()

    return (
        <div className="flex">
            <SidebarProvider>
                <SongEditSidebar />

                <div className="w-screen h-screen bg-neutral-900 pr-10 pl-2 pt-2 pb-8">
                    <div className="flex">
                        <SidebarTrigger className="mt-1" />
                        <Menubar className="w-fit ml-6">
                            <MenubarMenu>
                                <MenubarTrigger>File</MenubarTrigger>
                                <MenubarContent>
                                    <MenubarItem>
                                        New Tab <MenubarShortcut>⌘T</MenubarShortcut>
                                    </MenubarItem>
                                    <MenubarItem>New Window</MenubarItem>
                                    <MenubarSeparator />
                                        <MenubarItem>Share</MenubarItem>
                                    <MenubarSeparator />
                                    <MenubarItem>Print</MenubarItem>
                                </MenubarContent>
                            </MenubarMenu>
                        </Menubar>

                    </div>
                    <div>

                        <div className="w-full">
                        <p>song edit - map details {uuid}</p>
                        </div>
                    </div>
                </div>
            </SidebarProvider>
            
        </div>
    )
}
