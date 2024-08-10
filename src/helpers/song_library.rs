use std::{fs::{self, File}, io::{BufReader, Read}};

use crate::resources::song_library::SongMetadata;
use crate::helpers::persistence::get_songs_dir;


pub fn scan_song_library() -> Result<Vec<SongMetadata>, Box<dyn std::error::Error>> {
    let songs_dir = get_songs_dir().unwrap();
    let mut song_metadata_list: Vec<SongMetadata> = vec![];

    // Iterate over the entries in the base directory
    for entry in fs::read_dir(songs_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a directory
        if path.is_dir() {
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

    return Ok(song_metadata_list);
}
