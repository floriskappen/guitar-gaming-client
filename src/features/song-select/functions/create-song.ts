import { BaseDirectory, writeFile } from '@tauri-apps/plugin-fs';
import { createSongEmpty } from '../internal-api/create-song-empty';

export async function createSong(file: File) {
    // Read as array buffer
    const arrayBuffer = await file.arrayBuffer();

    // Convert to format necessary for Tauri's write function
    const fileContent = new Uint8Array(arrayBuffer)

    const uuid = await createSongEmpty()
    console.log(`going to write file in songs/${uuid}/audio.mp3`)
    await writeFile(`songs/${uuid}/audio.mp3`, fileContent, { baseDir: BaseDirectory.AppData })
}
