import { BaseDirectory, create } from '@tauri-apps/plugin-fs';
import { createSongEmpty } from '../internal-api/create-song-empty';

export async function createSong(file: File) {
    // Read as array buffer
    const arrayBuffer = await file.arrayBuffer();

    // Convert to format necessary for Tauri's write function
    const fileContent = new Uint8Array(arrayBuffer)

    const uuid = await createSongEmpty()
    const audioFile = await create(`songs/${uuid}/audio.mp3`, { baseDir: BaseDirectory.AppData })
    console.log(`going to write file in songs/${uuid}/audio.mp3`)
    await audioFile.write(fileContent)
    await audioFile.close()
}
