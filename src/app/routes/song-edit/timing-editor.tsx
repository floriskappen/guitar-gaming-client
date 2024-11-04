import { songEditAtom } from "@/features/song-edit/atoms/song-edit"
import { useAtom } from "jotai"
import { useParams } from "react-router-dom"

export const SongEditTimingEditor = () => {
    let { uuid } = useParams()
    const [song, setSong] = useAtom(songEditAtom)

    return (
        <div className="w-full px-8 ml-6">
            <p>song edit - bpm: {song?.bpm}</p>
        </div>
    )
}
