use bevy::prelude::*;

use super::song_library::SongMetadata;

#[derive(Resource, Debug)]
pub struct SongLoadedResource {
    pub metadata: Option<SongMetadata>,
    // pub audio: // TODO,
    // pub map: // TODO,
}

impl Default for SongLoadedResource {
    fn default() -> Self {
        Self {
            metadata: None
        }
    }
}

impl SongLoadedResource {
    pub fn load_song(&mut self, song_metadata: SongMetadata) {
        self.metadata = Some(song_metadata);
    }
}

