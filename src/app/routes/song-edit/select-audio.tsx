import { Button } from "@/components/ui/button"
import { MdChevronLeft, MdChevronRight } from "react-icons/md"
import { useNavigate, useParams } from "react-router-dom"

export const SongEditSelectAudioRoute = () => {
    let { uuid } = useParams()
    const navigate = useNavigate()

    return (
        <div className="w-screen h-screen bg-neutral-900 px-10 py-8">
            <div className="w-full flex justify-between">
                <Button variant="outline dashed" onClick={() => {
                    navigate('/song-select')
                }}>
                    <MdChevronLeft className="w-4 h-4" />
                    back to song select
                </Button>
                <Button variant="outline" onClick={() => {
                    navigate(`/song-edit/${uuid}/details`)
                }}>
                    continue
                    <MdChevronRight className="w-4 h-4" />
                </Button>
            </div>

            <div className="mt-16 flex justify-center w-full">
                <p>
                    song edit - select audio file {uuid}
                </p>
            </div>
        </div>
    )
}
