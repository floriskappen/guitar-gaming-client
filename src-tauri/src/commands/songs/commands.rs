use std::{fs::{self, File}, io::Write};

use uuid::Uuid;
use tauri::{AppHandle, Manager};

use crate::{commands::songs::helpers::get_songs as get_songs_helper, features::audio_feature_extraction::{audio_feature_extraction::get_tempo, helpers::waveform::get_or_generate_waveform_by_song_uuid, structs::waveform_data::WaveformData}};
use super::structs::Song;


#[tauri::command]
pub fn get_songs(app_handle: AppHandle, use_cache: bool) -> Vec<Song> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs = get_songs_helper(app_data_directory, use_cache);
    return songs;
}

#[tauri::command]
pub fn get_song_by_uuid(app_handle: AppHandle, uuid: String) -> Option<Song> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs = get_songs_helper(app_data_directory, true);
    return songs.iter().find(|&song| song.uuid == uuid).map(|song| song.clone());
}

// Creates a new song by generating a uuid, creating a directory for it with a metadata.json file
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
        duration_seconds: None,
        bpm: None
    };
    file.write_all(serde_json::to_string(&song_empty).expect("Failed to serialize JSON").as_bytes()).unwrap();

    return uuid_string;
}

#[tauri::command]
pub fn update_song_by_uuid(app_handle: AppHandle, uuid: String, updated_song_data: Song) -> Option<Song> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs_directory = app_data_directory.join("songs");

    let songs = get_songs_helper(app_data_directory, true);
    let song_option = songs.iter().find(|&song| song.uuid == uuid).map(|song| song.clone());

    if let Some(song) = song_option {
        let song_directory = songs_directory.join(song.uuid.clone());
        let metadata_path = song_directory.join("metadata.json");

        let mut file = File::create(metadata_path).unwrap();
        file.write_all(serde_json::to_string(&updated_song_data).expect("Failed to serialize JSON").as_bytes()).unwrap();
        return Some(updated_song_data)
    }

    return None
}

#[tauri::command]
pub fn delete_song_by_uuid(app_handle: AppHandle, uuid: String) {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs_directory = app_data_directory.join("songs");

    let songs = get_songs_helper(app_data_directory, true);
    let song_option = songs.iter().find(|&song| song.uuid == uuid).map(|song| song.clone());

    if let Some(song) = song_option {
        let song_directory = songs_directory.join(song.uuid.clone());
        fs::remove_dir_all(song_directory).expect("Failed to remove song directory");
    }

}

#[tauri::command]
pub async fn get_song_tempo_by_uuid(app_handle: AppHandle, uuid: String) -> Option<f64> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();
    let songs_directory = app_data_directory.join("songs");

    let songs = get_songs_helper(app_data_directory, true);
    let song_option = songs.iter().find(|&song| song.uuid == uuid).map(|song| song.clone());

    if let Some(song) = song_option {
        let song_directory = songs_directory.join(song.uuid.clone());
        let song_audio_directory = song_directory.join("audio.mp3");
        return Some(get_tempo(
            app_handle.path().resource_dir().unwrap(), song_audio_directory
        ).expect("error calculating tempo"));
    }
    return None
}


#[tauri::command]
pub async fn get_song_waveform_by_uuid(app_handle: AppHandle, uuid: String) -> Option<WaveformData> {
    let app_data_directory = app_handle.path().app_data_dir().unwrap();

    let waveform_data = get_or_generate_waveform_by_song_uuid(app_data_directory, uuid.clone())
        .expect(format!("Error generating waveform for song {}", uuid).as_str());

   return Some(waveform_data);
}

