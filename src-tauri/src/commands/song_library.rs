use crate::helpers::song_library::{load_from_disk, SongLibrary};


#[tauri::command]
pub fn get_song_library(refresh: bool) -> SongLibrary {
    let song_library = load_from_disk(refresh);
    return song_library;
}
