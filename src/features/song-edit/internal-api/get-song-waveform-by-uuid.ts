import { WaveformData } from "@/types/waveform_data"
import { invoke } from "@tauri-apps/api/core"

export const getSongWaveformByUuid = async (uuid: string): Promise<WaveformData | null> => {
    return await invoke<WaveformData | null>("get_song_waveform_by_uuid", { uuid })
}
