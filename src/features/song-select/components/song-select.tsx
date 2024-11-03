import { Button } from "@/components/ui/button"
import { Song } from "./song"
import { useEffect, useRef, useState } from "react"
import { getSongs } from "../internal-api/get-songs"

import { BiLoaderAlt } from "react-icons/bi";
import { useAtom, useSetAtom } from "jotai"
import { songsAtom } from "@/features/song-select/atoms/song-select"
import { useNavigate } from "react-router-dom";
import { DrawerTrigger } from "@/components/ui/drawer";
import { SelectAudio } from "@/features/song-edit/components/select-audio";
import { audioFileAtom } from "@/features/song-edit/atoms/song-edit";
import { createSong } from "@/features/song-select/functions/create-song";

export const SongSelect = () => {
    const [loading, setLoading] = useState(true)
    const [selectAudioOpen, setSelectAudioOpen] = useState(false)
    const [selectAudioLoading, setSelectAudioLoading] = useState(false)
    const [songs, setSongs] = useAtom(songsAtom)
    const getSongsStarted = useRef(false)

    const navigate = useNavigate()
    const setAudioFile = useSetAtom(audioFileAtom)

    useEffect(() => {
        if (!getSongsStarted.current) {
            getSongsStarted.current = true
            getSongs(true).then((data) => {
                setSongs(data)
                setLoading(false)
            })
        }
    }, [])

    return (
        <div>
            <div className="w-[600px]">
                <div className="w-full items-center flex justify-between">
                    <p>select a song</p>
                    <SelectAudio onSelect={async (file) => {
                        setAudioFile(file)
                        setSelectAudioLoading(true)
                        const uuid = await createSong(file)
                        navigate(`/song-edit/${uuid}/details`)
                    }} open={selectAudioOpen} loading={selectAudioLoading} setOpen={setSelectAudioOpen}>
                        <DrawerTrigger>
                            <Button variant="outline" onClick={() => setSelectAudioOpen(true)}>
                                map new song +
                            </Button>
                        </DrawerTrigger>
                    </SelectAudio>
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
                        songs?.map((song) => {
                            return <Song key={song.uuid} song={song} />
                        })
                    }

                    {
                        !loading && !songs?.length ? (
                            <p className="w-full text-center">no songs (yet!)</p>
                        ) : null
                    }
                </div>
            </div>
        </div>
    )
}
