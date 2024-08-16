use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_mod_billboard::{prelude::*, BillboardLockAxis};

use crate::{components::{button_minimal::spawn_button_minimal, song_timeline::spawn_song_timeline}, constants::ingame::{CAMERA_Y_RANGE, FRET_AMOUNT, FRET_CENTERS, STRING_CENTERS}, resources::song_loaded::SongLoadedResource, states::app_state::AppState};

use super::camera::spawn_camera;


#[derive(Component)]
pub struct SongPlayMarker;
#[derive(Component)]
pub struct BackButtonMarker;
#[derive(Component)]
pub struct SecondsPassedMarker;
#[derive(Component)]
pub struct Camera3D;

pub fn song_play_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    song_loaded: Res<SongLoadedResource>,
    mut clear_color: ResMut<ClearColor>
) {
    clear_color.0 = Color::srgb(0.10, 0.10, 0.10);

    // Content
    commands.spawn((
        SpatialBundle {
            ..Default::default()
        }, SongPlayMarker
    )).with_children(|builder| {
        // 3D camera
        spawn_camera(builder, Vec3 { x: 8.5, y: CAMERA_Y_RANGE[0], z: 10.6 });

        // Guitar neck model
        builder.spawn(SceneBundle {
            scene: asset_server.load("models/ingame/guitar_neck.glb#Scene0"),
            transform: Transform {
                rotation: Quat::from_rotation_x(90_f32.to_radians()),
                ..Default::default()
            },
            ..default()
        });

        // Song timeline
        spawn_song_timeline(builder, &mut meshes, &mut materials);
    
        let font = asset_server.load("fonts/IBMPlexMono-Regular.ttf");
    
        for fret_index in 0..FRET_AMOUNT {
            builder.spawn((BillboardTextBundle {
                transform: Transform {
                    translation: Vec3 { x: 0.0, y: FRET_CENTERS[fret_index], z: -0.35 },
                    rotation: Quat { w: 0.5, x: 0.5, y: -0.5, z: -0.5 },
                    scale: Vec3::splat(0.0085),
                    ..Default::default()
                },
                text: Text::from_sections([
                    TextSection {
                        value: format!("{}", fret_index+1),
                        style: TextStyle {
                            font_size: 28.0,
                            font: font.clone(),
                            color: Color::WHITE,
                        },
                    },
                ])
                .with_justify(JustifyText::Center),
                ..default()
            }, BillboardLockAxis{
                rotation: true,
                ..default()
            }));
        }
    
        // Directional 'sun' light
        builder.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::AMBIENT_DAYLIGHT * 1.5,
                shadows_enabled: false,
                shadow_depth_bias: 0.3,
                shadow_normal_bias: 0.3,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_euler(EulerRot::XYZ, (-16 as f32).to_radians(), (26 as f32).to_radians(), 0.0),
                ..default()
            },
            ..default()
        });
    });

    // UI
    commands.spawn((Camera2dBundle {
        camera: Camera {
            order: 0,
            ..Default::default()
        },
        ..Default::default()
    }, SongPlayMarker));
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    }, SongPlayMarker))
        .with_children(|builder| {

            // Header
            builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    padding: UiRect::bottom(Val::Px(32.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|builder| {
                // Back button
                spawn_button_minimal(
                    builder,
                    &asset_server,
                    "< back to song select",
                    BackButtonMarker
                );

                // Seconds passed
                builder.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).with_children(|builder| {
                    builder.spawn((TextBundle::from_section(
                        "0.00",
                        TextStyle {
                            font: asset_server.load("fonts/IBMPlexMono-Medium.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ), SecondsPassedMarker));
                });
        });

        // Content
        builder.spawn(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(24.),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
            .with_children(|builder|{
                // Title
                let song_metadata = song_loaded.metadata.clone().unwrap();
                builder.spawn(TextBundle::from_section(
                    format!(
                        "{} - {}",
                        song_metadata.artists.join(", "),
                        song_metadata.title.clone()
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
}

pub fn song_play_update(
    time: Res<Time>,
    back_button_query_interaction: Query<&Interaction, With<BackButtonMarker>>,
    mut seconds_passed_query: Query<&mut Text, With<SecondsPassedMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut song_loaded: ResMut<SongLoadedResource>,
) {
    for interaction in back_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::SongSelect);
        }
    }

    if let Some(mut song_progress) = song_loaded.progress.clone() {
        song_progress.timer.tick(time.delta());

        for mut text in seconds_passed_query.iter_mut() {
            text.sections[0].value = song_progress.timer.elapsed_secs().to_string()
        }

        song_loaded.progress = Some(song_progress);

    }

}

pub fn song_play_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SongPlayMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
