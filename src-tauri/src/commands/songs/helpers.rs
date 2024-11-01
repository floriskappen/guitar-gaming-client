use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const FILENAME: &str = "song_library.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    pub uuid: String,
    pub title: String,
    pub artists: Vec<String>,
    pub tuning: [String; 6],
    pub duration_seconds: f32,
}

fn get_songs_from_disk(
    app_data_directory: PathBuf,
) -> Result<Vec<Song>, Box<dyn std::error::Error>> {
    let songs_directory = app_data_directory.join("songs");
    let mut songs: Vec<Song> = vec![];

    // Iterate over the entries in the base directory
    for entry in fs::read_dir(songs_directory)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a directory
        if path.is_dir() {
            // Check if "audio.mp3" exists inside the directory
            let audio_path = path.join("audio.mp3");
            if !audio_path.exists() {
                println!("ERRPR: audio.mp3 not found in {:?}", path);
                continue;
            }

            // Create the path to the "metadata.json" file within the directory
            let metadata_path = path.join("metadata.json");

            // Check if the "metadata.json" file exists
            if metadata_path.exists() && metadata_path.is_file() {
                let mut file = File::open(metadata_path).expect("Failed to open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Failed to read file");

                // Deserialize the JSON contents into the serializable struct
                let song: Song =
                    serde_json::from_str(&contents).expect("Failed to deserialize JSON");

                // Add the deserialized metadata to the vector
                songs.push(song);
            }
        }
    }

    Ok(songs)
}

pub fn get_songs(app_data_directory: PathBuf, use_cache: bool) -> Vec<Song> {
    let songs_directory = app_data_directory.join("songs");
    let filepath = songs_directory.join(FILENAME);

    if filepath.exists() && use_cache {
        // Open the file and read its contents
        let mut file = File::open(filepath).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        // Deserialize the JSON contents into the serializable struct
        let serializable_configuration: Vec<Song> =
            serde_json::from_str(&contents).expect("Failed to deserialize JSON");
        println!("{:?}", serializable_configuration);
        return serializable_configuration;
    }

    return get_songs_from_disk(app_data_directory).unwrap()
}
