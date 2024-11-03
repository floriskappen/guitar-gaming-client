import { BaseDirectory, writeFile } from '@tauri-apps/plugin-fs';

export async function updateSongAudioFileByUuid(uuid: string, file: File) {
    // Read as array buffer
    const arrayBuffer = await file.arrayBuffer();

    // Convert to format necessary for Tauri's write function
    const fileContent = new Uint8Array(arrayBuffer)

    console.log(`going to write file in songs/${uuid}/audio.mp3`)
    await writeFile(`songs/${uuid}/audio.mp3`, fileContent, { baseDir: BaseDirectory.AppData })
}
