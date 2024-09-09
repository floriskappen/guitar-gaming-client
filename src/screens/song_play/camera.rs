use bevy::prelude::*;

use crate::{components::song_note::SongNote, constants::ingame::{CAMERA_Y_RANGE, FRET_AMOUNT}};

#[derive(Component)]
pub struct Camera3DMarker;

pub fn spawn_camera(
    builder: &mut ChildBuilder,
    starting_position: Vec3,
) {
    builder.spawn((Camera3dBundle {
        // projection: Projection::Perspective(PerspectiveProjection {
        //     ..Default::default()
        // }),
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedHorizontal(19.0),
            ..Default::default()
        }),
        transform: Transform {
            translation: starting_position,
            rotation: Quat { w: 0.386419,  x: 0.223099, y: 0.447467, z: 0.775036 },
            ..Default::default()
        },
        camera: Camera {
            order: 1,
            ..Default::default()
        },
        ..default()
    }, Camera3DMarker));
}

pub fn camera_system(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera3DMarker>>,
        Query<&SongNote, With<SongNote>>
    )>,
    time: Res<Time>,
) {
    let mut song_note_total_fret_index = 0;
    let mut song_note_amount = 0;
    for song_note in set.p1().iter() {
        song_note_amount += 1;
        song_note_total_fret_index += song_note.note_event.fret_index;
    }
    if song_note_amount > 0 {
        let song_note_average_fret_index = song_note_total_fret_index / song_note_amount;
    
        /*
        Map the fret index to a camera Z position using linear interploration
        - source_min = 0 (fret index 0)
        - source_max = FRET_AMOUNT
        - target_min = CAMERA_Y_RANGE[0]
        - target_max = CAMERA_Y_RANGE[1]
    
        formula:
            mapped_value = target_min + (value - source_min / source_max - source_min) * (target_max - target_min)
        */
        let new_y = CAMERA_Y_RANGE[0] + (song_note_average_fret_index as f32 / FRET_AMOUNT as f32) * (CAMERA_Y_RANGE[1] - CAMERA_Y_RANGE[0]);
        
        for mut transform in set.p0().iter_mut() {
            transform.translation.y = transform.translation.y + time.delta_seconds() * (new_y - transform.translation.y);
        }
    }
}
