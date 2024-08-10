use std::{fs::File, io::Read};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::helpers::{persistence::get_data_dir, song_library::scan_song_library};

const FILENAME: &str = "song_library.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongMetadata {
    pub uuid: String,
    pub title: String,
    pub artists: Vec<String>
}

#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct SongLibraryResource {
    pub songs: Vec<SongMetadata>
}

impl Default for SongLibraryResource {
    fn default() -> Self {
        Self { songs: scan_song_library().unwrap() }
    }
}

impl SongLibraryResource {
    pub fn save_to_disk(&self) {
        let directory = get_data_dir().unwrap();
        let filepath = directory.join(FILENAME);
        let file = File::create(filepath).expect("Failed to create file");
        serde_json::to_writer(file, self).expect("Failed to write JSON to file");
    }
    pub fn load_from_disk() -> Self {
        let directory = get_data_dir().unwrap();
        let filepath = directory.join(FILENAME);

        if filepath.exists() {
            // Open the file and read its contents
            let mut file = File::open(filepath).expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read file");
    
            // Deserialize the JSON contents into the serializable struct
            let serializable_configuration: SongLibraryResource =
                serde_json::from_str(&contents).expect("Failed to deserialize JSON");
            println!("{:?}", serializable_configuration);
            return serializable_configuration
        }

        return SongLibraryResource::default();
    }

    pub fn find_by_id(&self, uuid: &String) -> Option<&SongMetadata> {
        return self.songs.iter().find(|song_metadata| &song_metadata.uuid == uuid);
    }
}
