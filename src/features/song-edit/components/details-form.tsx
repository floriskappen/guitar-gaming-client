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
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
 
const formSchema = z.object({
    artists: z.array(
        z.string(), {
            message: 'please add at least one artist or write "unknown"'
        }
    ).min(1, 'please add at least one artist or write "unknown"'),
    title: z.string().min(1, {
        message: "song name must be at least 1 character",
    }),
    tuning: z.array(
        z.string().regex(/^[a-gA-G]#?[0-9]$/, {
            message: "each tuning must be a note followed by an optional '#' and an octave number, like 'e2', 'f#3', or 'd3'",
        })
    ).min(6, 'please set the tuning for every string')
})

export const SongDetailsForm = () => {
    const tuningPresets = [
        ["e2", "a2", "d3", "g3", "b3", "e4"],
        ["d2", "a2", "d3", "g3", "b3", "e4"],
        ["d2", "a2", "d3", "g3", "a3", "d4"],
        ["d2", "g2", "d3", "g3", "b3", "d4"],
        ["d2", "g2", "d3", "f#3", "a3", "d4"],
        ["c#2", "g#2", "c#3", "f#3", "a#3", "d#4"],
    ]
    const [song, setSong] = useAtom(songEditAtom)
    const [saveLoading, setSaveLoading] = useState(false)
    const [selectAudioOpen, setSelectAudioOpen] = useState(false)
    const [selectAudioLoading, setSelectAudioLoading] = useState(false)
    const [currentArtistInputValue, setCurrentArtistInputValue] = useState<string>("")
    const [artists, setArtists] = useState<string[]>(song?.artists || [])
    const [tuningString, setTuningString] = useState<string>(() => {
        if (!song || song.tuning === null) {
            return JSON.stringify(tuningPresets[0])
        }

        if (song && song.tuning && tuningPresets.some(preset => JSON.stringify(preset) === JSON.stringify(song.tuning))) {
            return JSON.stringify(song.tuning)
        }
        return "custom"
    })

    let { uuid } = useParams()

    const navigate = useNavigate()

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            title: song?.title || "",
            artists: song?.artists || [],
            tuning: song?.tuning || tuningPresets[0],
        },
    })
    const { setValue, getValues } = form;


    async function onSubmit(values: z.infer<typeof formSchema>) {
        if (uuid) {
            setSaveLoading(true)
            const newSong = await updateSongByUuid(uuid, {
                artists: values.artists,
                title: values.title,
                uuid,
                duration_seconds: null,
                tuning: values.tuning,
                bpm: null
            })
            if (newSong) {
                setSong(newSong)
            }

            setSaveLoading(false)

            navigate(`/song-edit/${uuid}/timing-editor`)
        }
    }

    function addArtist(artist: string) {
        const newArtists = [...artists, artist]
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
                                                return <div key={artist} className="px-3 py-0.5 border-4 border-neutral-700 flex items-center text-xs">
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

                <FormField
                    control={form.control}
                    name="tuning"
                    render={() => (
                        <FormItem>
                            <FormLabel>guitar tuning</FormLabel>
                            <FormControl>
                                <div>
                                    <Select onValueChange={(value) => {
                                        setTuningString(value)
                                        if (value !== "custom") {
                                            setValue("tuning", JSON.parse(value))
                                        }
                                    }} value={tuningString}>
                                        <SelectTrigger className="max-w-[480px]">
                                            <SelectValue placeholder="choose from common tuning" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectGroup>
                                                <SelectItem value={JSON.stringify(tuningPresets[0])}>standard ({tuningPresets[0].join(", ")})</SelectItem>
                                                <SelectItem value={JSON.stringify(tuningPresets[1])}>drop d ({tuningPresets[1].join(", ")})</SelectItem>
                                                <SelectItem value={JSON.stringify(tuningPresets[2])}>DADGAD ({tuningPresets[2].join(", ")})</SelectItem>
                                                <SelectItem value={JSON.stringify(tuningPresets[3])}>open g ({tuningPresets[3].join(", ")})</SelectItem>
                                                <SelectItem value={JSON.stringify(tuningPresets[4])}>open d ({tuningPresets[4].join(", ")})</SelectItem>
                                                <SelectItem value={JSON.stringify(tuningPresets[5])}>open c# ({tuningPresets[5].join(", ")})</SelectItem>
                                                <SelectItem value="custom">custom</SelectItem>
                                            </SelectGroup>
                                        </SelectContent>
                                    </Select>

                                    {
                                        tuningString === "custom" ? (
                                            <div className="flex max-w-[480px] space-x-2 mt-2">
                                                {
                                                    Array.from([0, 1, 2, 3, 4, 5]).map((index) => {
                                                        return <Input key={index} value={getValues().tuning[index]} onChange={(e) => {
                                                            let newTuning = getValues().tuning
                                                            newTuning[index] = e.currentTarget.value
                                                            setValue("tuning", newTuning)
                                                        }} />
                                                    })
                                                }
                                            </div>
                                        ) : null
                                    }
                                </div>
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <div>
                    <div className="flex space-x-2 items-center mt-16">
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
                </div>
            </form>
        </Form>
    )
}
