use bevy::prelude::*;

use crate::{constants::ingame::{NOTE_END_LOCATION_X, NOTE_START_LOCATION_X}, resources::song_loaded::SongLoadedResource};

use super::song_note::{spawn_song_note, SongNote};

#[derive(Component)]
pub struct SongNotes;

pub fn spawn_song_notes(
    builder: &mut ChildBuilder,
) {
    builder.spawn((SpatialBundle {
        ..Default::default()
    }, SongNotes));
}


pub fn manage_song_notes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut song_loaded: ResMut<SongLoadedResource>,
    song_notes_wrapper_query: Query<Entity, With<SongNotes>>,
    mut song_notes_spawned_query: Query<(Entity, &mut Transform, &SongNote), With<SongNote>>,
) {
    if let Some(mut song_progress) = song_loaded.progress.clone() {
        let current_time = song_progress.timer.elapsed().as_secs_f32();

        let index = 0;
        while let Some(note_event) = song_progress.notes_remaining.get(index) {
            if note_event.start_time_seconds <= current_time {
                // Spawn note
                for entity in song_notes_wrapper_query.iter() {
                    info!("spawning note");
                    commands.entity(entity).with_children(|builder| {
                        spawn_song_note(builder, &asset_server, note_event.clone());
                    });
                }

                song_progress.notes_remaining.remove(index);
            } else {
                break;
            }
        }

        for (entity, mut note_transform, song_note) in song_notes_spawned_query.iter_mut() {
            let progress = current_time - song_note.note_event.start_time_seconds / song_note.note_event.duration_seconds;

            if progress < 1.0 {
                let new_position_x = NOTE_START_LOCATION_X + progress * (NOTE_END_LOCATION_X - NOTE_START_LOCATION_X);
                note_transform.translation = Vec3 {
                    x: new_position_x,
                    y: note_transform.translation.y,
                    z: note_transform.translation.z,
                }
            } else {
                info!("despawning note");
                commands.entity(entity).despawn_recursive();
            }
        }

        song_progress.timer.tick(time.delta());


        song_loaded.progress = Some(song_progress);
    }
}
