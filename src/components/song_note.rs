use bevy::prelude::*;

use crate::{constants::ingame::{FRET_CENTERS, NOTE_START_LOCATION_X, STRING_CENTERS}, helpers::song_notes::NoteEvent};

#[derive(Component)]
pub struct SongNote {
    pub note_event: NoteEvent
}

pub fn spawn_song_note(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    note_event: NoteEvent,
) {
    let note_model = asset_server.load("models/ingame/note.glb#Scene0");
    builder.spawn((SceneBundle {
        scene: note_model,
        transform: Transform {
            translation: Vec3 { x: NOTE_START_LOCATION_X, y: FRET_CENTERS[note_event.fret_index], z: STRING_CENTERS[note_event.string_index] },
            rotation: Quat::from_rotation_x(90_f32.to_radians()),
            ..Default::default()
        },
        
        ..default()
    }, SongNote { note_event: note_event }));
}
