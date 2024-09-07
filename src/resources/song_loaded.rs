use std::{fs::File, io::BufReader};

use bevy::prelude::*;
use uuid::Uuid;

use crate::helpers::{persistence::get_songs_dir, song_notes::NoteEvent};

use super::song_library::SongMetadata;

#[derive(Debug, Clone)]
pub struct NoteEventWithScore {
    missed: bool,
    note_event: NoteEvent
}

#[derive(Debug, Clone)]
pub struct SongLoadedProgress {
    pub timer: Timer,
    pub notes_with_score: Vec<NoteEventWithScore>,
    pub previous_onset_secs: f32
}

#[derive(Resource, Debug)]
#[derive(Default)]
pub struct SongLoadedResource {
    pub metadata: Option<SongMetadata>,
    pub audio_path: Option<String>,
    pub notes: Option<Vec<NoteEvent>>,
    pub progress: Option<SongLoadedProgress>,
}


impl SongLoadedResource {
    pub fn load_song(&mut self, song_metadata: SongMetadata) {
        self.metadata = Some(song_metadata.clone());

        // Set the audio filepath
        let song_directory = get_songs_dir().unwrap().join(
            format!("{}/audio.mp3", song_metadata.uuid)
        );
        self.audio_path = Some(song_directory.to_str().unwrap().to_string());

        // TODO: properly load notes
        let mut note_events = vec![
            NoteEvent {
                start_time_seconds: 1.0,
                duration_seconds: 1.0,
                string_index: 0,
                fret_index: 9,
            },
            NoteEvent {
                start_time_seconds: 2.0,
                duration_seconds: 1.0,
                string_index: 1,
                fret_index: 10,
            },
            NoteEvent {
                start_time_seconds: 3.0,
                duration_seconds: 1.0,
                string_index: 2,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 4.0,
                duration_seconds: 1.0,
                string_index: 3,
                fret_index: 9,
            },
            NoteEvent {
                start_time_seconds: 5.0,
                duration_seconds: 1.0,
                string_index: 4,
                fret_index: 10,
            },
            NoteEvent {
                start_time_seconds: 6.0,
                duration_seconds: 1.0,
                string_index: 5,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 7.0,
                duration_seconds: 1.0,
                string_index: 5,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 8.0,
                duration_seconds: 1.0,
                string_index: 5,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 9.0,
                duration_seconds: 1.0,
                string_index: 5,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 10.0,
                duration_seconds: 1.0,
                string_index: 5,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 11.0,
                duration_seconds: 1.0,
                string_index: 4,
                fret_index: 11,
            },
            NoteEvent {
                start_time_seconds: 12.0,
                duration_seconds: 1.0,
                string_index: 4,
                fret_index: 8,
            },
            NoteEvent {
                start_time_seconds: 13.0,
                duration_seconds: 1.0,
                string_index: 2,
                fret_index: 8,
            },
            NoteEvent {
                start_time_seconds: 14.0,
                duration_seconds: 1.0,
                string_index: 2,
                fret_index: 8,
            },
            NoteEvent {
                start_time_seconds: 15.0,
                duration_seconds: 1.0,
                string_index: 2,
                fret_index: 8,
            },
        ];


        // Sort the note events
        note_events.sort_by(|a, b| a.start_time_seconds.partial_cmp(&b.start_time_seconds).unwrap());

        self.notes = Some(note_events.clone());

        self.progress = Some(SongLoadedProgress {
            timer: Timer::from_seconds(song_metadata.duration_seconds, TimerMode::Once),
            notes_with_score: vec![],
            previous_onset_secs: 0.0,
        })
    }
}

