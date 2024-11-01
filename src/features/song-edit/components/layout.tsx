import { Sidebar } from "@/features/song-edit/components/sidebar"
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { Menubar } from "@/features/song-edit/components/menubar"
import { useEffect } from "react";
import { songEditAtom } from "../atoms/song-edit";
import { useSetAtom } from "jotai";


export const Layout = ({ children }: { children: React.ReactNode }) => {
    const setSongEdit = useSetAtom(songEditAtom);

    useEffect(() => {
        return () => {
            setSongEdit(null)
        };
    }, []);

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
                        {/* <div>
                            {
                                showWaveform && songEdit?.audio ? (
                                    <Wavesurfer />
                                ) : null
                            }
                        </div> */}
                        <div className="w-full px-8 ml-6">
                            { children }
                        </div>
                    </div>
                </div>
            </SidebarProvider>
        </div>
    )
}
