
use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};

use crate::{constants::ingame::{FRET_CENTERS, STRING_CENTERS, STRING_COLORS}, helpers::song_notes::NoteEvent, resources::song_loaded::SongLoadedResource};

#[derive(Event)]
pub struct SongNoteTriggeredEvent(pub SongNote);

#[derive(Component, Debug, Clone)]
pub struct SongNote {
    pub note_event: NoteEvent,
    pub triggered: bool,
}

#[derive(Component)]
pub struct SongNoteFront {
    pub initial_color: Color
}

#[derive(Component)]
pub struct SongNoteFrontFadeout {
    pub progress: Timer
}

#[derive(Component)]
pub struct GuideStartMarker;

#[derive(Component)]
pub struct TrailMarker {
    length: f32
}

pub fn spawn_song_note(
    builder: &mut ChildBuilder,
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
        builder.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.6, 1.1, 0.26))),
            material: materials.add(StandardMaterial {
                base_color: STRING_COLORS[note_event.string_index],
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.9,
                metallic: 0.0,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3 { x: length, y: 0.0, z: STRING_CENTERS[note_event.string_index] },
                ..Default::default()
            },
            ..default()
        }, SongNoteFront { initial_color: STRING_COLORS[note_event.string_index] }));

        // Note trail
        builder.spawn((PbrBundle {
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
        }, TrailMarker { length }));


        if note_event.string_index > 0 {
            // Guide start
            builder.spawn((PbrBundle {
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
            }, GuideStartMarker));
            builder.spawn((PbrBundle {
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
            }, GuideStartMarker));

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

pub fn update_song_note(
    time: Res<Time>,
    mut commands: Commands,
    mut event_song_note_triggered: EventReader<SongNoteTriggeredEvent>,
    mut song_notes_query: Query<(&mut SongNote, &Children)>,
    front_query: Query<(Entity, &SongNoteFront), With<SongNoteFront>>,
    guide_start_marker_query: Query<Entity, With<GuideStartMarker>>,
    mut set: ParamSet<(
        // Trail Marker
        Query<(Entity, &TrailMarker, &Handle<Mesh>, &mut Transform), With<TrailMarker>>,
        // Front with fadeout
        Query<(Entity, &SongNoteFront, &mut SongNoteFrontFadeout, &mut Transform), (With<SongNoteFront>, With<SongNoteFrontFadeout>)>
    )>,
    material_handle_query: Query<&mut Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    song_loaded: Res<SongLoadedResource>,
) {
    // Fadeout front
    for (entity, song_note_front, mut song_note_front_timeout, mut transform) in set.p1().iter_mut() {
        song_note_front_timeout.progress.tick(time.delta());
        let t = (song_note_front_timeout.progress.elapsed_secs() / song_note_front_timeout.progress.duration().as_secs_f32()).min(1.0);

        if let Ok(material_handle) = material_handle_query.get(entity) {
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = song_note_front.initial_color.mix(&song_note_front.initial_color.with_alpha(0.0), t);
            }
        }

        transform.scale = Vec3::splat(1.0).lerp(Vec3::splat(1.3), t);

        if song_note_front_timeout.progress.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Process just triggered song notes
    for event in event_song_note_triggered.read() {
        for (mut song_note, children) in song_notes_query.iter_mut() {
            if song_note.note_event.equals(&event.0.note_event) {
                for &child in children.iter() {
                    // Fade out the front
                    if let Ok((song_note_front, _)) = front_query.get(child) {
                        commands.entity(song_note_front).insert(
                            SongNoteFrontFadeout {
                                progress: Timer::from_seconds(0.15, TimerMode::Once)
                            }
                        );
                    }

                }
                song_note.triggered = true;
                break
            }
        }
    }

    // Progress triggered song notes
    let song_elapsed_seconds = song_loaded.progress.as_ref().unwrap().timer.elapsed_secs();
    for (song_note, children) in song_notes_query.iter() {
        if song_note.note_event.start_time_seconds < song_elapsed_seconds {

            for &child in children.iter() {
                // Remove the starting guide marker
                if let Ok(guide_start) = guide_start_marker_query.get(child) {
                    commands.entity(guide_start).despawn_recursive()
                }

                if song_note.triggered {
                    // Make the trail smaller
                    if let Ok((entity, trail_marker, mesh_handle, mut transform)) = set.p0().get_mut(child) {
                        if let Some(mesh) = meshes.get_mut(mesh_handle) {
                            let original_length = trail_marker.length;
                            let t = (((song_elapsed_seconds - song_note.note_event.start_time_seconds) / song_note.note_event.duration_seconds)).min(1.0).max(0.0);
                            let new_length = original_length * (1.0 - t);


                            transform.translation.x = 0.0 - (original_length * t);

                            *mesh = PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(new_length, 0.1) }, subdivisions: 0 }.build();
                        }
                    }
                }
            }

        }
    }
}
