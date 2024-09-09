use bevy::prelude::*;
use bevy_mod_billboard::{prelude::*, BillboardLockAxis};

use crate::{components::{button_minimal::spawn_button_minimal, song_note::{SongNote, SongNoteTriggeredEvent}, song_timeline::spawn_song_timeline}, constants::ingame::{CAMERA_Y_RANGE, FRET_AMOUNT, FRET_CENTERS, STRING_COLORS}, helpers::{input_device::AudioStream, persistence::get_songs_dir}, resources::{configuration::ConfigurationResource, input_device::InputDeviceResource, output_audio_song::{AudioCommand, OutputAudioControllerSong}, song_loaded::SongLoadedResource}, states::app_state::AppState};

use super::camera::spawn_camera;


#[derive(Component)]
pub struct SongPlayMarker;
#[derive(Component)]
pub struct BackButtonMarker;
#[derive(Component)]
pub struct SecondsPassedMarker;
#[derive(Component)]
pub struct DebugOnsetMarker;

pub fn song_play_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    song_loaded: Res<SongLoadedResource>,
    mut clear_color: ResMut<ClearColor>,
    mut input_device: ResMut<InputDeviceResource>,
    output_audio_song: Res<OutputAudioControllerSong>,
    configuration: Res<ConfigurationResource>,
) {
    // Set up the input audio stream
    input_device.audio_stream_main = Some(AudioStream::new(configuration.device.clone().unwrap(), configuration.selected_device_channels.clone(), 1024).unwrap());

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
                rotation: Quat::from_euler(EulerRot::XYZ, (-16_f32).to_radians(), 26_f32.to_radians(), 0.0),
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

                builder.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
                        ..Default::default()
                    },
                    DebugOnsetMarker
                ));
            });
        });

    // Play the audio
    let _ = output_audio_song.sender.send(AudioCommand::Play(song_loaded.audio_path.as_ref().unwrap().clone()));
}

pub fn song_play_update(
    time: Res<Time>,
    back_button_query_interaction: Query<&Interaction, With<BackButtonMarker>>,
    mut seconds_passed_query: Query<&mut Text, With<SecondsPassedMarker>>,
    mut debug_onset_marker: Query<&mut BackgroundColor, With<DebugOnsetMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut song_loaded: ResMut<SongLoadedResource>,
    input_device: Res<InputDeviceResource>,
    mut song_notes_query: Query<&SongNote>,
    mut event_song_note_triggered: EventWriter<SongNoteTriggeredEvent>,
) {
    for interaction in back_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::SongSelect);
        }
    }

    if let Some(song_progress) = song_loaded.progress.as_mut() {
        song_progress.timer.tick(time.delta());
        let elapsed_secs = song_progress.timer.elapsed_secs();

        for mut text in seconds_passed_query.iter_mut() {
            text.sections[0].value = elapsed_secs.to_string()
        }

        if let Some(audio_stream) = &input_device.audio_stream_main {
            // Debounce - exact value yet to be determined after more experimentation
            if song_progress.previous_onset_secs + 0.1 < elapsed_secs {
                if let Ok(has_onset) = audio_stream.get_onset() {
                    if has_onset {
                        song_progress.previous_onset_secs = elapsed_secs;

                        for song_note in song_notes_query.iter_mut() {
                            if song_note.triggered {
                                continue
                            }

                            // If the timing is somewhat close (tweak later)
                            if song_note.note_event.start_time_seconds > elapsed_secs - 0.2 && song_note.note_event.start_time_seconds < elapsed_secs + 0.2 {
                                event_song_note_triggered.send(SongNoteTriggeredEvent(song_note.clone()));
                            }
                        }

                        for mut debug_onset_el in debug_onset_marker.iter_mut() {
                            *debug_onset_el = BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 1.0));
                        }
                    }
                }
            }
        } else {
            error!("no audio stream :c")
        }

        if song_progress.previous_onset_secs + 0.08 < elapsed_secs {
            for mut debug_onset_el in debug_onset_marker.iter_mut() {
                *debug_onset_el = BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0));
            }
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
