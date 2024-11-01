import { Button } from "@/components/ui/button"
import { MdChevronLeft } from "react-icons/md";
import { SongSelect } from "@/features/song-select/components/song-select";
import { RefreshSongListButton } from "@/features/song-select/components/refresh-song-list-button";
import { useNavigate } from "react-router-dom";


export const SongSelectRoute = () => {
    const navigate = useNavigate()

    return (
        <div className="w-screen h-screen bg-neutral-900 px-10 py-8">
            <div className="w-full flex justify-between">
                <Button variant="outline dashed" onClick={() => {
                    navigate('/input-device-select')
                }}>
                    <MdChevronLeft className="w-4 h-4" />
                    change input device
                </Button>
                <RefreshSongListButton />
            </div>

            <div className="mt-16 flex justify-center w-full">
                <SongSelect />
            </div>
        </div>
    )
}
