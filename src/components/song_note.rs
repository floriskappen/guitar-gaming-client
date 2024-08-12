
use bevy::prelude::*;

use crate::{constants::ingame::{FRET_CENTERS, TIMELINE_LENGTH, STRING_CENTERS, STRING_COLORS}, helpers::song_notes::NoteEvent, resources::{configuration::ConfigurationResource, song_loaded::SongLoadedResource}};

#[derive(Component)]
pub struct SongNote {
    pub note_event: NoteEvent,
}

pub fn spawn_song_note(
    builder: &mut ChildBuilder,
    configuration: &Res<ConfigurationResource>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    note_event: NoteEvent,
) {
    let length = note_event.duration_seconds * configuration.approach_rate;

    builder.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(length, 1.1, 0.26))),
        material: materials.add(StandardMaterial {
            base_color: STRING_COLORS[note_event.string_index],
            perceptual_roughness: 0.9,
            metallic: 0.0,
            ..Default::default()
        }),
        transform: Transform {
            translation: Vec3 { x: -TIMELINE_LENGTH, y: FRET_CENTERS[note_event.fret_index], z: STRING_CENTERS[note_event.string_index] },
            ..Default::default()
        },
        ..default()
    }, SongNote { note_event }));
}

pub fn move_song_notes(
    mut commands: Commands,
    song_loaded: Res<SongLoadedResource>,
    configuration: Res<ConfigurationResource>,
    mut song_notes_query: Query<(Entity, &mut Transform, &SongNote)>,
) {
    if let Some(song_progress) = song_loaded.progress.clone() {
        let current_time = song_progress.timer.elapsed().as_secs_f32();

        for (entity, mut note_transform, song_note) in song_notes_query.iter_mut() {
            let length = song_note.note_event.duration_seconds * configuration.approach_rate;
            let progress = (current_time + (TIMELINE_LENGTH / configuration.approach_rate) - song_note.note_event.start_time_seconds / song_note.note_event.duration_seconds) / (TIMELINE_LENGTH / configuration.approach_rate);

            if progress < 1.0 {
                let new_position_x = -TIMELINE_LENGTH + progress * ((length/2.0) + TIMELINE_LENGTH);
                note_transform.translation = Vec3 {
                    x: new_position_x,
                    y: note_transform.translation.y,
                    z: note_transform.translation.z,
                }
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
