import { Button } from "@/components/ui/button"
import { ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger } from "@/components/ui/context-menu"
import { SongMetadata } from "@/features/song-select/types/song-library";
import { useRef } from "react"
import { BsThreeDots } from "react-icons/bs"
import { FaPencilAlt, FaTrashAlt } from "react-icons/fa";
import { FaListUl } from "react-icons/fa6";
import { useNavigate } from "react-router-dom";

export type SongProps = {
    metadata: SongMetadata
}

export const Song = ({ metadata }: SongProps) => {
    const trigger = useRef<HTMLDivElement>(null)

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

    return (
        <ContextMenu>
            <ContextMenuTrigger ref={trigger}>
                <Button variant="outline" className="h-12 w-full p-0 items-center flex mt-2" onClick={() => {
                    navigate(`/song-play/${metadata.uuid}`)
                }}>
                    <div className="bg-neutral-700 w-6 h-full shrink-0"></div>
                    <p className="px-4 w-full text-start">{metadata.artists.join(", ")} - {metadata.title}</p>
                    <div onClick={(event) => {
                        event.preventDefault()
                        event.stopPropagation()
                        manuallyOpenContextMenu(event.currentTarget)
                    }} className="flex items-center justify-center h-full px-4 w-16 shrink-0 cursor-pointer">
                        <BsThreeDots className="w-6 h-6 text-neutral-500 shrink-0" />
                    </div>
                </Button>
            </ContextMenuTrigger>
            <ContextMenuContent data-state="open" className="w-64">
                <ContextMenuItem className="flex w-full items-center justify-between">
                    <p>edit</p>
                    <FaPencilAlt className="w-3 h-3 text-neutral-300" />
                </ContextMenuItem>
                <ContextMenuItem className="flex w-full items-center justify-between">
                    <p>add to collection</p>
                    <FaListUl className="w-3 h-3 text-neutral-300" />
                </ContextMenuItem>
                <ContextMenuItem className="flex w-full items-center justify-between">
                    <p>delete</p>
                    <FaTrashAlt className="w-3 h-3 text-neutral-300" />
                </ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    )
}
