
use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

use crate::{constants::ingame::{FRET_CENTERS, TIMELINE_LENGTH, STRING_CENTERS, STRING_COLORS}, helpers::song_notes::NoteEvent, resources::{configuration::ConfigurationResource, song_loaded::SongLoadedResource}};

#[derive(Component)]
pub struct SongNote {
    pub note_event: NoteEvent,
    pub triggered: bool,
}

pub fn spawn_song_note(
    builder: &mut ChildBuilder,
    configuration: &Res<ConfigurationResource>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    note_event: NoteEvent,
    length: f32,
    x_position: f32,
) {

    builder.spawn((SpatialBundle {
        transform: Transform {
            translation: Vec3 { x: x_position, y: FRET_CENTERS[note_event.fret_index], z: 0.0 },
            ..Default::default()
        },
        ..Default::default()
    }, SongNote { note_event: note_event.clone(), triggered: false })).with_children(|builder| {

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
            mesh: meshes.add(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(length, 0.1) }, subdivisions: 0 }.build()),
            material: materials.add(StandardMaterial {
                base_color: STRING_COLORS[note_event.string_index].with_luminance(0.8),
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
