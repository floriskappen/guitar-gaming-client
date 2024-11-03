import { invoke } from "@tauri-apps/api/core"

export const getSongTempoByUuid = async (uuid: string): Promise<number | null> => {
    return await invoke<number | null>("get_song_tempo_by_uuid", { uuid })
}
