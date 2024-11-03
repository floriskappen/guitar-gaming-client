import { Song } from "@/types/song"
import { invoke } from "@tauri-apps/api/core"

export const updateSongByUuid = async (uuid: string, updatedSongData: Song): Promise<Song | null> => {
    const updatedSong = await invoke<Song | null>("update_song_by_uuid", { uuid, updatedSongData })
    return updatedSong
}
