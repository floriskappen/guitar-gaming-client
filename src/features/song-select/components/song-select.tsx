import { Button } from "@/components/ui/button"
import { Song } from "./song"
import { useEffect, useState } from "react"
import { getSongLibrary } from "../internal-api/get-song-library"

import { BiLoaderAlt } from "react-icons/bi";
import { useAtom, useSetAtom } from "jotai"
import { songLibraryAtom } from "../atoms/song-select"
import { useNavigate } from "react-router-dom";
import { DrawerTrigger } from "@/components/ui/drawer";
import { SelectAudio } from "@/features/song-edit/components/select-audio";
import { audioFileAtom } from "@/features/song-edit/atoms/song-edit";

export const SongSelect = () => {
    const [loading, setLoading] = useState(true)
    const [selectAudioOpen, setSelectAudioOpen] = useState(false)
    const [songLibrary, setSongLibrary] = useAtom(songLibraryAtom)

    const navigate = useNavigate()
    const setAudioFile = useSetAtom(audioFileAtom)

    useEffect(() => {
        getSongLibrary(false).then((data) => {
            setSongLibrary(data)
            setLoading(false)
        })
    }, [])

    return (
        <div>
            <SelectAudio onSelect={(file) => {
                setAudioFile(file)
                navigate("/song-edit/new/details")
            }} open={selectAudioOpen} setOpen={setSelectAudioOpen}>
                <div className="w-[600px]">
                    <div className="w-full items-center flex justify-between">
                        <p>select a song</p>
                        <DrawerTrigger>
                            <Button variant="outline" onClick={() => setSelectAudioOpen(true)}>
                                map new song +
                            </Button>
                        </DrawerTrigger>
                    </div>

                    <div className="mt-8">
                        {
                            loading ? (
                                <div className="w-full flex justify-center py-4">
                                    <BiLoaderAlt className="animate-spin" />
                                </div>
                            ) : null
                        }

                        {
                            songLibrary?.songs?.map((songMetadata) => {
                                return <Song key={songMetadata.uuid} metadata={songMetadata} />
                            })
                        }

                        {
                            !loading && !songLibrary?.songs ? (
                                <p>no songs</p>
                            ) : null
                        }
                    </div>
                </div>
            </SelectAudio>
        </div>
    )
}
