use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

use crate::{constants::ingame::{FRET_CENTERS, TIMELINE_LENGTH}, resources::{configuration::ConfigurationResource, song_loaded::SongLoadedResource}};

use super::components::note::{spawn_note, Note};

#[derive(Component)]
pub struct Notes;

pub fn spawn_timeline(
    builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Spawn song notes wrapper
    builder.spawn((SpatialBundle {
        ..Default::default()
    }, Notes));

    // Spawn fret guides
    for (fret_index, fret_center) in FRET_CENTERS.iter().enumerate() {
        if fret_index == 0 {
            builder.spawn(PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(30.0, 0.02) }, subdivisions: 0 }.build()),
                material: materials.add(StandardMaterial {
                    // base_color_texture: Some(asset_server.load("path/to/your/image.png")),
                    base_color: Color::srgb(0.4, 0.4, 0.4),
                    perceptual_roughness: 0.9,
                    metallic: 0.0,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: Vec3::new(-30.0, fret_center - 0.6, 0.0),
                    // rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    rotation: Quat::from_rotation_x(90_f32.to_radians()),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }

        builder.spawn(
            PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(30.0, 0.02) }, subdivisions: 0 }.build()),
                material: materials.add(StandardMaterial {
                    // base_color_texture: Some(asset_server.load("path/to/your/image.png")),
                    base_color: Color::srgb(0.4, 0.4, 0.4),
                    perceptual_roughness: 0.9,
                    metallic: 0.0,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: Vec3::new(-30.0, fret_center + 0.6, 0.0),
                    // rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    rotation: Quat::from_rotation_x(90_f32.to_radians()),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}


pub fn update_timeline(
    mut commands: Commands,
    song_loaded: Res<SongLoadedResource>,
    notes_wrapper_query: Query<Entity, With<Notes>>,
    mut notes_query: Query<(Entity, &mut Transform, &Note)>,
    configuration: Res<ConfigurationResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(song_progress) = song_loaded.progress.clone() {
        let current_time = song_progress.timer.elapsed().as_secs_f32();
        let note_events = song_loaded.notes.clone().unwrap();

        let notes_wrapper = notes_wrapper_query.iter().next().unwrap();

        for note_event in note_events {
            // If the note has already passed
            if (note_event.start_time_seconds + note_event.duration_seconds) < current_time {
                continue;
            }

            // If we have reached a note that has yet to be spawned we can skip it.
            // Since the notes are sorted by time we know the same will be true for all note events to come, so we can break out of the loop
            if note_event.start_time_seconds - configuration.approach_rate > current_time {
                break
            }

            let existing_song_note = notes_query.iter_mut().find(|(_, _, note)| {
                note.note_event.equals(&note_event)
            });

            if existing_song_note.is_none() {
                commands.entity(notes_wrapper).with_children(|builder| {
                    spawn_note(&configuration, builder, &mut meshes, &mut materials, note_event);
                });
            }
        }
    }
}
