import { invoke } from "@tauri-apps/api/core"

export const createSongEmpty = async (): Promise<string> => {
    const uuid = await invoke<string>("create_song_empty")
    return uuid
}
