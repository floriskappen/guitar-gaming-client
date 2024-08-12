use std::f32::consts::PI;

use bevy::{color::palettes::css::ORANGE_RED, pbr::CascadeShadowConfigBuilder, prelude::*, render::mesh::PlaneMeshBuilder};
use bevy_mod_billboard::{prelude::*, BillboardLockAxis};

use crate::{components::{button_minimal::spawn_button_minimal, song_note::spawn_song_note, song_notes::spawn_song_notes}, constants::ingame::{FRET_AMOUNT, FRET_CENTERS, NOTE_START_LOCATION_X, STRING_CENTERS}, resources::song_loaded::SongLoadedResource, states::app_state::AppState};


#[derive(Component)]
pub struct SongPlayMarker;
#[derive(Component)]
pub struct BackButtonMarker;

pub fn song_play_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    song_loaded: Res<SongLoadedResource>,
    mut clear_color: ResMut<ClearColor>
) {
    clear_color.0 = Color::srgb(0.10, 0.10, 0.10);

    // 3D
    commands.spawn((Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov: (25 as f32).to_radians(),
            ..Default::default()
        }),
        transform: Transform {
            translation: Vec3 { x: 16.0, y: -6.0, z: 4.5 },
            rotation: Quat { x: 0.426, y: 0.508, z: 0.574, w: 0.481 },
            ..Default::default()
        },
        camera: Camera {
            order: 1,
            
            ..Default::default()
        },
        ..default()
    }, SongPlayMarker));

    // Content
    commands.spawn((
        SpatialBundle {
            ..Default::default()
        }, SongPlayMarker
    )).with_children(|builder| {
        // Guitar neck model
        builder.spawn(SceneBundle {
            scene: asset_server.load("models/ingame/guitar_neck.glb#Scene0"),
            transform: Transform {
                rotation: Quat::from_rotation_x(90_f32.to_radians()),
                ..Default::default()
            },
            ..default()
        });
    
        // Notes
        spawn_song_notes(builder);
    
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

        // Test
        for (fret_index, fret_center) in FRET_CENTERS.iter().enumerate() {
            if fret_index == 0 {
                builder.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(30.0, 0.02) }, subdivisions: 0 }.build())),
                    material: materials.add(StandardMaterial {
                        // base_color_texture: Some(asset_server.load("path/to/your/image.png")),
                        base_color: Color::WHITE,
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
                    mesh: meshes.add(Mesh::from(PlaneMeshBuilder { plane: Plane3d { normal: Dir3::Y, half_size: Vec2::new(30.0, 0.02) }, subdivisions: 0 }.build())),
                    material: materials.add(StandardMaterial {
                        // base_color_texture: Some(asset_server.load("path/to/your/image.png")),
                        base_color: Color::WHITE,
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
    
        // Directional 'sun' light
        builder.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: light_consts::lux::AMBIENT_DAYLIGHT * 2.,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            // The default cascade config is designed to handle large scenes.
            // As this example has a much smaller world, we can tighten the shadow
            // bounds for better visual quality.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 10.0,
                ..default()
            }
            .into(),
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
                    }
                ));
            });
        });
}

pub fn song_play_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    back_button_query_interaction: Query<&Interaction, With<BackButtonMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in back_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::SongSelect);
        }
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
