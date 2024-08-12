use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

use crate::{constants::ingame::{FRET_CENTERS, TIMELINE_LENGTH}, resources::{configuration::ConfigurationResource, song_loaded::SongLoadedResource}};

use super::song_note::spawn_song_note;

#[derive(Component)]
pub struct SongNotes;

pub fn spawn_song_timeline(
    builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Spawn song notes wrapper
    builder.spawn((SpatialBundle {
        ..Default::default()
    }, SongNotes));

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


pub fn manage_song_timeline(
    mut commands: Commands,
    mut song_loaded: ResMut<SongLoadedResource>,
    song_notes_wrapper_query: Query<Entity, With<SongNotes>>,
    configuration: Res<ConfigurationResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(mut song_progress) = song_loaded.progress.clone() {
        let current_time = song_progress.timer.elapsed().as_secs_f32();

        let index = 0;
        while let Some(note_event) = song_progress.notes_remaining.get(index) {
            if note_event.start_time_seconds - (TIMELINE_LENGTH / configuration.approach_rate) <= current_time {
                // Spawn note
                for entity in song_notes_wrapper_query.iter() {
                    commands.entity(entity).with_children(|builder| {
                        spawn_song_note(builder, &configuration, &mut meshes, &mut materials, note_event.clone());
                    });
                }

                song_progress.notes_remaining.remove(index);
            } else {
                break;
            }
        }

        song_loaded.progress = Some(song_progress);
    }
}
