use std::{fs::{self, File}, io::Read};

use serde::{Deserialize, Serialize};

use super::persistence::{get_data_dir, get_songs_dir};

const FILENAME: &str = "song_library.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongMetadata {
    pub uuid: String,
    pub title: String,
    pub artists: Vec<String>,
    pub tuning: [String; 6],
    pub duration_seconds: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SongLibrary {
    pub songs: Vec<SongMetadata>
}
impl Default for SongLibrary {
    fn default() -> Self {
        Self { songs: get_song_metadata_list().unwrap() }
    }
}

fn get_song_metadata_list() -> Result<Vec<SongMetadata>, Box<dyn std::error::Error>> {
    let songs_dir = get_songs_dir().unwrap();
    let mut song_metadata_list: Vec<SongMetadata> = vec![];

    // Iterate over the entries in the base directory
    for entry in fs::read_dir(songs_dir)? {
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
                file.read_to_string(&mut contents).expect("Failed to read file");
        
                // Deserialize the JSON contents into the serializable struct
                let song_metadata: SongMetadata =
                    serde_json::from_str(&contents).expect("Failed to deserialize JSON");

                // Add the deserialized metadata to the vector
                song_metadata_list.push(song_metadata);
            }
        }
    }

    Ok(song_metadata_list)
}

pub fn load_from_disk(refresh: bool) -> SongLibrary {
    let directory = get_data_dir().unwrap();
    let filepath = directory.join(FILENAME);

    if filepath.exists() && !refresh {
        // Open the file and read its contents
        let mut file = File::open(filepath).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        // Deserialize the JSON contents into the serializable struct
        let serializable_configuration: SongLibrary =
            serde_json::from_str(&contents).expect("Failed to deserialize JSON");
        println!("{:?}", serializable_configuration);
        return serializable_configuration
    }

    SongLibrary::default()
}

