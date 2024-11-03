import { Song } from "@/types/song"
import { invoke } from "@tauri-apps/api/core"

export const getSongByUuid = async (uuid: string): Promise<Song | null> => {
    const song = await invoke<Song | null>("get_song_by_uuid", { uuid })
    return song
}
