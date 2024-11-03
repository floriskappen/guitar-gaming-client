import { getSongTempoByUuid } from "@/features/song-edit/internal-api/get-song-tempo-by-uuid"
import { useEffect } from "react"
import { useParams } from "react-router-dom"

export const SongEditTimingEditor = () => {
    let { uuid } = useParams()

    useEffect(() => {
        getSongTempoByUuid(uuid!)
    })

    return (
        <div className="w-full px-8 ml-6">
            <p>song edit - timing editor: {uuid}</p>
        </div>
    )   
}
