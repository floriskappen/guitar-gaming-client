use bevy::prelude::*;
use uuid::Uuid;

use crate::helpers::song_notes::NoteEvent;

use super::song_library::SongMetadata;

#[derive(Debug, Clone)]
pub struct NoteEventWithScore {
    missed: bool,
    note_event: NoteEvent
}

#[derive(Debug, Clone)]
pub struct SongLoadedProgress {
    pub timer: Timer,
    pub notes_remaining: Vec<NoteEvent>,
    pub notes_with_score: Vec<NoteEventWithScore>,
}

#[derive(Resource, Debug)]
#[derive(Default)]
pub struct SongLoadedResource {
    pub metadata: Option<SongMetadata>,
    // pub audio: // TODO,
    pub notes: Option<Vec<NoteEvent>>,
    pub progress: Option<SongLoadedProgress>
}


impl SongLoadedResource {
    pub fn load_song(&mut self, song_metadata: SongMetadata) {
        self.metadata = Some(song_metadata.clone());

        // TODO: load audio file

        // TODO: properly load notes
        let mut notes_test = vec![
            NoteEvent {
                start_time_seconds: 6.0,
                duration_seconds: 1.0,
                string_index: 0,
                fret_index: 0,
                identifier: Uuid::new_v4(),
            },
            NoteEvent {
                start_time_seconds: 6.5,
                duration_seconds: 1.0,
                string_index: 1,
                fret_index: 1,
                identifier: Uuid::new_v4(),
            },
            NoteEvent {
                start_time_seconds: 7.0,
                duration_seconds: 1.0,
                string_index: 2,
                fret_index: 2,
                identifier: Uuid::new_v4(),
            },
            NoteEvent {
                start_time_seconds: 7.5,
                duration_seconds: 1.0,
                string_index: 3,
                fret_index: 0,
                identifier: Uuid::new_v4(),
            },
            NoteEvent {
                start_time_seconds: 8.0,
                duration_seconds: 1.0,
                string_index: 4,
                fret_index: 1,
                identifier: Uuid::new_v4(),
            },
            NoteEvent {
                start_time_seconds: 8.5,
                duration_seconds: 1.0,
                string_index: 5,
                fret_index: 2,
                identifier: Uuid::new_v4(),
            },
        ];
        notes_test.sort_by(|a, b| a.start_time_seconds.partial_cmp(&b.start_time_seconds).unwrap());
        self.notes = Some(notes_test.clone());

        self.progress = Some(SongLoadedProgress {
            timer: Timer::from_seconds(song_metadata.duration_seconds, TimerMode::Once),
            notes_remaining: notes_test.clone(),
            notes_with_score: vec![],
        })
    }
}

