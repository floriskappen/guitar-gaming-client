import { Song } from "@/types/song"
import { invoke } from "@tauri-apps/api/core"

export const deleteSongByUuid = async (uuid: string) => {
    await invoke<Song | null>("delete_song_by_uuid", { uuid })
}
