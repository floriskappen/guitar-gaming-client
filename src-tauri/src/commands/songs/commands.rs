use std::{fs::{self, File}, io::Write};

use uuid::Uuid;
use tauri::{AppHandle, Manager};

use crate::commands::songs::helpers::{get_songs as get_songs_helper, Song};

#[tauri::command]
pub fn get_songs(app_handle: AppHandle, use_cache: bool) -> Vec<Song> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs = get_songs_helper(app_data_directory, use_cache);
    return songs;
}

// Creates a new song by generating a uuid, creating a directory for it with an empty metadata.json file
#[tauri::command]
pub fn create_song_empty(app_handle: AppHandle) -> String {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs_directory = app_data_directory.join("songs");

    // Create the UUID
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string();

    // Create the directory for the song
    let song_directory = songs_directory.join(uuid_string.clone());
    println!("Creating {}", song_directory.to_str().unwrap());
    fs::create_dir_all(song_directory.clone()).unwrap();

    // Create an empty metadata.json file
    let metadata_path = song_directory.join("metadata.json");
    let mut file = File::create(metadata_path).unwrap();
    let song_empty = Song {
        uuid: uuid_string.clone(),
        title: None,
        artists: None,
        tuning: None,
        duration_seconds: None
    };
    file.write_all(serde_json::to_string(&song_empty).expect("Failed to serialize JSON").as_bytes()).unwrap();

    return uuid_string;
}
