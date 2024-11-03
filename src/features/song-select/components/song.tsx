import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle } from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button"
import { ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "@/components/ui/context-menu"
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { Song as SongType } from "@/types/song";
import { useRef, useState } from "react"
import { BsThreeDots } from "react-icons/bs"
import { FaPencilAlt, FaTrashAlt } from "react-icons/fa";
import { FaListUl } from "react-icons/fa6";
import { useNavigate } from "react-router-dom";
import { twMerge } from "tailwind-merge";
import { deleteSongByUuid } from "../internal-api/delete-song-by-uuid";
import { getSongs } from "../internal-api/get-songs";
import { songsAtom } from "../atoms/song-select";
import { useSetAtom } from "jotai";

export type SongProps = {
    song: SongType
}

export const Song = ({ song }: SongProps) => {
    const trigger = useRef<HTMLDivElement>(null)
    const [deleteSongAlertOpen, setDeleteSongAlertOpen] = useState(false)
    const [deleteSongLoading, setDeleteSongLoading] = useState(false)
    const setSongs = useSetAtom(songsAtom)

    const navigate = useNavigate()

    function manuallyOpenContextMenu(target: HTMLElement) {
        trigger?.current?.dispatchEvent(
            new MouseEvent("contextmenu", {
                bubbles: true,
                clientX: target.getBoundingClientRect().x,
                clientY: target.getBoundingClientRect().y + 32,
            })
        )
    }

    const songIsPlayable = !!song.title

    return (
        <ContextMenu>
            <Tooltip>
                <ContextMenuTrigger ref={trigger}>
                    <TooltipTrigger className="w-full mt-2">
                        <Button variant="outline" className={twMerge(
                            "h-12 w-full p-0 items-center flex",
                            !songIsPlayable ? "border-neutral-700 hover:border-neutral-700" : ""
                        )} onClick={() => {
                            if (songIsPlayable) {
                                navigate(`/song-play/${song.uuid}`)
                            }
                        }}>
                            <div className="bg-neutral-700 w-6 h-full shrink-0"></div>
                            <p className="px-4 w-full text-start">
                                {
                                    song.artists && song.artists.length > 0 ? (
                                        <span>
                                            {song.artists.join(", ")}
                                        </span>
                                    ) : (
                                        <span className="text-neutral-400">
                                            unknown
                                        </span>
                                    )
                                }
                                <span> - </span>
                                {
                                    song.title ? (
                                        <span>{song.title}</span>
                                    ) : (
                                        <span className="text-neutral-400">unknown</span>
                                    )
                                }
                                </p>
                            <div onClick={(event) => {
                                event.preventDefault()
                                event.stopPropagation()
                                manuallyOpenContextMenu(event.currentTarget)
                            }} className="flex items-center justify-center h-full px-4 w-16 shrink-0 cursor-pointer">
                                <BsThreeDots className="w-6 h-6 text-neutral-500 shrink-0" />
                            </div>
                        </Button>
                    </TooltipTrigger>
                </ContextMenuTrigger>
                {
                    !songIsPlayable ? (
                        <TooltipContent>start mapping this song in order to be able to play it</TooltipContent>
                    ) : null
                }
            </Tooltip>
            <ContextMenuContent data-state="open" className="w-64">
                <ContextMenuItem className="flex w-full items-center justify-between" onClick={() => {
                    navigate(`/song-edit/${song.uuid}/details`)
                }}>
                    <p>edit</p>
                    <FaPencilAlt className="w-3 h-3 text-neutral-300" />
                </ContextMenuItem>
                <ContextMenuItem className="flex w-full items-center justify-between">
                    <p>add to collection</p>
                    <FaListUl className="w-3 h-3 text-neutral-300" />
                </ContextMenuItem>
                <ContextMenuItem className="flex w-full items-center justify-between" onClick={() => setDeleteSongAlertOpen(true)}>
                    <p>delete</p>
                    <FaTrashAlt className="w-3 h-3 text-neutral-300" />
                </ContextMenuItem>
            </ContextMenuContent>

            <AlertDialog open={deleteSongAlertOpen}>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>are you sure?</AlertDialogTitle>
                        <AlertDialogDescription>
                            you are about to delete this song. this action cannot be undone. are you sure?
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel onClick={() => setDeleteSongAlertOpen(false)}>cancel</AlertDialogCancel>
                        <AlertDialogAction disabled={deleteSongLoading} onClick={async () => {
                            setDeleteSongLoading(true)
                            await deleteSongByUuid(song.uuid)
                            setDeleteSongAlertOpen(false)
                            getSongs(false).then((data) => {
                                setSongs(data)
                            })
                        }}>delete song</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        </ContextMenu>
    )
}
