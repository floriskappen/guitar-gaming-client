import { BaseDirectory, readFile } from '@tauri-apps/plugin-fs';

export async function getSongAudioBlobByUuid(uuid: string): Promise<Blob> {
    const fileData = await readFile(`songs/${uuid}/audio.mp3`, { baseDir: BaseDirectory.AppData })

    // Convert the data to a Blob and create an Object URL
    const blob = new Blob([new Uint8Array(fileData)], { type: "audio/mpeg" })
    return blob
}
