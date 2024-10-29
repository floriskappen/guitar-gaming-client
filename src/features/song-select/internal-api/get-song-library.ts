import { SongLibrary } from "@/features/song-select/types/song-library"
import { invoke } from "@tauri-apps/api/core"

export const getSongLibrary = async (refresh: boolean): Promise<SongLibrary> => {
    const songLibrary = await invoke<SongLibrary>("get_song_library", { refresh })
    return songLibrary
}
