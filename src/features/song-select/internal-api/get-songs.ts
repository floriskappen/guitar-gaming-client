import { Song } from "@/types/song"
import { invoke } from "@tauri-apps/api/core"

export const getSongs = async (useCache: boolean): Promise<Song[]> => {
    const songs = await invoke<Song[]>("get_songs", { useCache })
    return songs
}
