import { Sidebar } from "@/features/song-edit/components/sidebar"
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { Menubar } from "@/features/song-edit/components/menubar"
import { useEffect, useRef, useState } from "react";
import { songEditAtom } from "../atoms/song-edit";
import { useSetAtom } from "jotai";
import { useNavigate, useParams, Outlet } from "react-router-dom";
import { getSongByUuid } from "@/internal-api/get-song-by-uuid";
import { AlertDialog, AlertDialogAction, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/components/ui/alert-dialog";
import { BiLoaderAlt } from "react-icons/bi";
import { toast } from "sonner";
import { getSongTempoByUuid } from "../internal-api/get-song-tempo-by-uuid";
import { updateSongByUuid } from "../internal-api/update-song-by-uuid";
import { Audio } from "./audio";
import { AudioProgressBar } from "./audio-progress-bar";

export const Layout = () => {
    const [loading, setLoading] = useState(true)
    const [noUuidAlertOpen, setNoUuidAlertOpen] = useState(false)
    const setSongEdit = useSetAtom(songEditAtom);
    const bpmDetectionStarted = useRef(false);

    let { uuid } = useParams()
    const navigate = useNavigate()

    useEffect(() => {
        const fetchSongData = async () => {
            if (uuid) {
                let song = await getSongByUuid(uuid)
                setSongEdit(song)
                setLoading(false)

                if (!bpmDetectionStarted.current && !song?.bpm) {
                    bpmDetectionStarted.current = true
                    toast("detecting song bpm...")
                    const bpm = await getSongTempoByUuid(uuid!)
                    toast("bpm detection complete", {
                        description: `song seems to be ${bpm} bpm`
                    })
                    song = await updateSongByUuid(song!.uuid, {
                        ...song!,
                        bpm
                    })
                    setSongEdit(song)
                }
            } else {
                setNoUuidAlertOpen(true)
            }
        }

        fetchSongData()

        return () => {
            setSongEdit(null)
        };
    }, []);

    return (
        <div className="flex bg-neutral-900">
            <AlertDialog open={noUuidAlertOpen}>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>something went wrong</AlertDialogTitle>
                        <AlertDialogDescription>
                            you are trying to edit a song, but the application did not provide the song's identifier.
                            this means we do not know which song you are talking about and thus cannot edit it.
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogAction onClick={() => {
                            navigate("/song-select")
                        }}>back to song select</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>

            {
                loading ? (
                    <div className="w-screen h-screen flex items-center justify-center">
                            <p className="mr-4">loading song...</p>
                            <BiLoaderAlt className="animate-spin" />
                    </div>
                ) : (
                    <SidebarProvider>
                        <Sidebar />

                        <div className="w-screen h-screen bg-neutral-900 flex flex-col overflow-hidden justify-between">
                            <div className="pt-2 pb-b">
                                <div className="flex pl-2">
                                    <SidebarTrigger className="mt-1" />
                                    <Menubar />

                                </div>
                                <div className="h-full">
                                    <Audio />
                                    <div className="w-full px-8 ml-6">
                                        <Outlet />
                                    </div>
                                </div>
                            </div>
                            <div>
                                <AudioProgressBar />
                            </div>
                        </div>
                    </SidebarProvider>
                )
            }
        </div>
    )
}
