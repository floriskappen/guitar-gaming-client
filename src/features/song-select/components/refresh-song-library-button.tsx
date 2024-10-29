import { Button } from "@/components/ui/button"
import { IoRefresh } from "react-icons/io5"
import { getSongLibrary } from "../internal-api/get-song-library"
import { useSetAtom } from "jotai"
import { songLibraryAtom } from "../atoms/song-select"

export const RefreshSongLibraryButton = () => {
    const setSongLibrary = useSetAtom(songLibraryAtom)

    return (
        <Button variant="outline dashed" onClick={() => {
            getSongLibrary(true).then((data) => {
                setSongLibrary(data)
            })
        }}>
            <IoRefresh className="w-4 h-4" />
            refresh song library
        </Button>
    )
}
