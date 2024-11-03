import { Song } from "@/types/song"
import { invoke } from "@tauri-apps/api/core"

export const getSongTempoByUuid = async (uuid: string) => {
    await invoke<Song | null>("get_song_tempo_by_uuid", { uuid })
}
