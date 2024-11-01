import { Button } from "@/components/ui/button"
import { IoRefresh } from "react-icons/io5"
import { getSongs } from "../internal-api/get-songs"
import { useSetAtom } from "jotai"
import { songsAtom } from "../atoms/song-select"

export const RefreshSongListButton = () => {
    const setSongs = useSetAtom(songsAtom)

    return (
        <Button variant="outline dashed" onClick={() => {
            getSongs(false).then((data) => {
                setSongs(data)
            })
        }}>
            <IoRefresh className="w-4 h-4" />
            refresh song list
        </Button>
    )
}
