use tauri::{AppHandle, Manager};

use crate::commands::songs::helpers::{get_songs as get_songs_helper, Song};

#[tauri::command]
pub fn get_songs(app_handle: AppHandle, use_cache: bool) -> Vec<Song> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs = get_songs_helper(app_data_directory, use_cache);
    return songs;
}
