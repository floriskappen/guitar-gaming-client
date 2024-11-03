import { z } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { Button } from "@/components/ui/button"
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { Input } from "@/components/ui/input"
import { SelectAudio } from "./select-audio"
import { songEditAtom } from "../atoms/song-edit"
import { useAtom } from "jotai"
import { useState } from "react"
import { DrawerTrigger } from "@/components/ui/drawer"
import { IoClose } from "react-icons/io5";
import { BiPlus } from "react-icons/bi"
import { useNavigate, useParams } from "react-router-dom"
import { updateSongByUuid } from "../internal-api/update-song-by-uuid"
import { updateSongAudioFileByUuid } from "@/features/functions/update-song-audio-file-by-uuid"
 
const formSchema = z.object({
    artists: z.array(
        z.string(), {
            message: 'please add at least one artist or write "unknown"'
        }
    ).min(1, 'please add at least one artist or write "unknown"'),
    title: z.string().min(1, {
        message: "song name must be at least 1 character",
    }),
})

export const SongDetailsForm = () => {
    const [song, setSong] = useAtom(songEditAtom)
    const [saveLoading, setSaveLoading] = useState(false)
    const [selectAudioOpen, setSelectAudioOpen] = useState(false)
    const [selectAudioLoading, setSelectAudioLoading] = useState(false)
    const [currentArtistInputValue, setCurrentArtistInputValue] = useState<string>("")
    const [artists, setArtists] = useState<string[]>(song?.artists || [])

    let { uuid } = useParams()

    const navigate = useNavigate()

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            title: song?.title || "",
            artists: song?.artists || []
        },
    })
    const { setValue } = form;

    async function onSubmit(values: z.infer<typeof formSchema>) {
        if (uuid) {
            setSaveLoading(true)
            const newSong = await updateSongByUuid(uuid, {
                artists: values.artists,
                title: values.title,
                uuid,
                duration_seconds: null,
                tuning: null
            })
            if (newSong) {
                setSong(newSong)
            }

            setSaveLoading(false)

            navigate(`/song-edit/${uuid}/timing-editor`)
        }
    }

    function addArtist(artist: string) {
        let newArtists = artists
        newArtists.push(artist)
        setArtists(newArtists)
        setValue("artists", newArtists)
        setCurrentArtistInputValue("")
    }
    

    return (
        <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8 mt-8">
                <FormField
                    control={form.control}
                    name="artists"
                    render={() => (
                        <FormItem>
                            <FormLabel>artist(s)</FormLabel>
                            <FormControl>
                                <div>
                                    <div className="flex space-x-2 mb-1">
                                        {
                                            artists.map((artist) => {
                                                return <div key={artist} className="px-4 py-1 border-4 border-neutral-700 flex items-center text-xs">
                                                    <p>{artist}</p>
                                                    <div className="ml-3 -mr-1 py-1 cursor-pointer" onClick={() => {
                                                        const newArtists = artists.filter((currentArtist) => currentArtist !== artist)
                                                        setArtists(newArtists)
                                                        setValue("artists", newArtists)
                                                    }}>
                                                        <IoClose />
                                                    </div>
                                                </div>
                                            })
                                        }
                                    </div>
                                    <div className="flex space-x-1 items-center">
                                        <Input value={currentArtistInputValue} onInput={(event) => setCurrentArtistInputValue(event.currentTarget.value)} placeholder="darude" className="max-w-[430px]" onKeyDown={(event) => {
                                            if (event.key === "Enter" && currentArtistInputValue.length > 0) {
                                                addArtist(currentArtistInputValue)
                                            }
                                        }} />
                                        <Button disabled={currentArtistInputValue.length == 0} variant={"ghost"} onClick={(e) => {
                                            e.stopPropagation()
                                            e.preventDefault()

                                            addArtist(currentArtistInputValue)
                                        }}><BiPlus className="w-4 h-4" /></Button>
                                    </div>
                                </div>

                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="title"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>song name</FormLabel>
                            <FormControl>
                                <Input placeholder="sandstorm" {...field} className="max-w-[480px]" />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <div className="flex space-x-2 items-center">
                    <Button type="submit" disabled={saveLoading}>save</Button>

                    <SelectAudio onSelect={async (newAudioFile) => {
                        setSelectAudioLoading(true)
                        await updateSongAudioFileByUuid(song!.uuid, newAudioFile)
                        setSelectAudioOpen(false)
                        setSelectAudioLoading(false)
                    }} open={selectAudioOpen} setOpen={setSelectAudioOpen} loading={selectAudioLoading}>
                        <DrawerTrigger>
                            <Button variant="secondary" onClick={(e) => {
                                e.preventDefault()
                                setSelectAudioOpen(true)
                            }}>change audio file</Button>
                        </DrawerTrigger>
                    </SelectAudio>
                </div>
            </form>
        </Form>
    )
}
