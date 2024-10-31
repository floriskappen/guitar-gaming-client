import { Button } from "@/components/ui/button"
import { Song } from "./song"
import { useEffect, useState } from "react"
import { getSongLibrary } from "../internal-api/get-song-library"

import { BiLoaderAlt } from "react-icons/bi";
import { useAtom } from "jotai"
import { songLibraryAtom } from "../atoms/song-select"
import { useNavigate } from "react-router-dom";

export const SongSelect = () => {
    const [loading, setLoading] = useState(true)
    const [songLibrary, setSongLibrary] = useAtom(songLibraryAtom)

    const navigate = useNavigate()

    useEffect(() => {
        getSongLibrary(false).then((data) => {
            setSongLibrary(data)
            setLoading(false)
        })
    }, [])

    return (
        <div className="w-[600px]">
            <div className="w-full items-center flex justify-between">
                <p>select a song</p>
                <Button variant="outline" onClick={() => {
                    navigate("/song-edit/new/select-audio")
                }}>
                    map new song +
                </Button>
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
    )
}
