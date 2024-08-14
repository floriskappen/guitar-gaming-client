
use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

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
    let length = (note_event.duration_seconds * configuration.approach_rate)/2.0;

    builder.spawn((SpatialBundle {
        transform: Transform {
            translation: Vec3 { x: -TIMELINE_LENGTH, y: FRET_CENTERS[note_event.fret_index], z: 0.0 },
            ..Default::default()
        },
        ..Default::default()
    }, SongNote { note_event: note_event.clone() })).with_children(|builder| {

        // Note front
        builder.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.6, 1.1, 0.26))),
            material: materials.add(StandardMaterial {
                base_color: STRING_COLORS[note_event.string_index],
                perceptual_roughness: 0.9,
                metallic: 0.0,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3 { x: length, y: 0.0, z: STRING_CENTERS[note_event.string_index] },
                ..Default::default()
            },
            ..default()
        });

        // Note trail
        builder.spawn(PbrBundle {
            mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(length, 0.55) }, subdivisions: 0 }.build()),
            material: materials.add(StandardMaterial {
                base_color: STRING_COLORS[note_event.string_index].with_luminance(0.8).with_alpha(0.6),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.9,
                
                metallic: 0.0,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, STRING_CENTERS[note_event.string_index]),
                rotation: Quat::from_rotation_x(90_f32.to_radians()),
                ..Default::default()
            },
            ..Default::default()
        });


        if note_event.string_index > 0 {
            // Guide start
            builder.spawn(PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(STRING_CENTERS[note_event.string_index]/2.0, 0.04) }, subdivisions: 0 }.build()),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.4, 0.4),
                    perceptual_roughness: 0.9,
                    metallic: 0.0,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: Vec3::new(length, 0.0, STRING_CENTERS[note_event.string_index]/2.0),
                    rotation: Quat::from_rotation_y(90_f32.to_radians()),
                    ..Default::default()
                },
                ..Default::default()
            });
            builder.spawn(PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new((FRET_CENTERS[1]-FRET_CENTERS[0])/2.0, 0.04) }, subdivisions: 0 }.build()),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.4, 0.4),
                    perceptual_roughness: 0.9,
                    metallic: 0.0,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: Vec3::new(length, 0.0, 0.0),
                    rotation: Quat::from_euler(EulerRot::XYZ, 0_f32.to_radians(), 90_f32.to_radians(), (90_f32).to_radians()),
                    ..Default::default()
                },
                ..Default::default()
            });

            // Guide end
            builder.spawn(PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(STRING_CENTERS[note_event.string_index]/2.0, 0.04) }, subdivisions: 0 }.build()),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.4, 0.4),
                    perceptual_roughness: 0.9,
                    metallic: 0.0,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: Vec3::new(-length, 0.0, STRING_CENTERS[note_event.string_index]/2.0),
                    rotation: Quat::from_rotation_z((90_f32).to_radians()),
                    ..Default::default()
                },
                ..Default::default()
            });
            builder.spawn(PbrBundle {
                mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new((FRET_CENTERS[1]-FRET_CENTERS[0])/2.0, 0.04) }, subdivisions: 0 }.build()),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.4, 0.4, 0.4),
                    perceptual_roughness: 0.9,
                    metallic: 0.0,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: Vec3::new(-length, 0.0, 0.0),
                    rotation: Quat::from_euler(EulerRot::XYZ, 0_f32.to_radians(), 90_f32.to_radians(), (90_f32).to_radians()),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    });
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
            let length = (song_note.note_event.duration_seconds * configuration.approach_rate)/2.0;
            let progress = (current_time + (TIMELINE_LENGTH / configuration.approach_rate) - song_note.note_event.start_time_seconds / song_note.note_event.duration_seconds) / (TIMELINE_LENGTH / configuration.approach_rate);

            if progress < 1.0 {
                let new_position_x = -TIMELINE_LENGTH + progress * (length + TIMELINE_LENGTH);
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
