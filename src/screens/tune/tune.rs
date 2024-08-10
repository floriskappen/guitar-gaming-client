use bevy::prelude::*;

use crate::{components::button_minimal::spawn_button_minimal, helpers::{input_device::AudioStream, tuning::{cents_off_from_pitch, note_from_pitch, octave_from_note, NOTE_STRINGS}}, resources::{configuration::ConfigurationResource, input_device::InputDeviceResource}, states::app_state::AppState};

#[derive(Component)]
pub struct CentsMarker;

#[derive(Component)]
pub struct NoteMarker;

#[derive(Component)]
pub struct TuneMarker;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct ContinueButton;

const COLOR_PITCH_GOOD: Color = Color::srgb(0.06, 0.88, 0.07);
const COLOR_PITCH_BAD: Color = Color::srgb(0.88, 0.06, 0.07);

pub fn tune_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut input_device: ResMut<InputDeviceResource>,
    configuration: Res<ConfigurationResource>,
) {
    input_device.audio_stream_main = Some(AudioStream::new(configuration.device.clone().unwrap(), configuration.selected_device_channels.clone(), 16384).unwrap());

    commands.spawn((Camera2dBundle::default(), TuneMarker));
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
        ..Default::default()
    }, TuneMarker))
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
                    "< go back",
                    BackButton
                );

                // Continue button
                spawn_button_minimal(
                    builder,
                    &asset_server,
                    "continue >",
                    ContinueButton
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
                builder.spawn(TextBundle::from_section(
                    "confirm tuning",
                    TextStyle {
                        font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    }
                ));

                // Tune information
                builder.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(4.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).with_children(|builder| {
                    builder.spawn((TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/IBMPlexMono-Bold.ttf"),
                            font_size: 18.0,
                            color: Color::WHITE,
                        }
                    ), NoteMarker));
                    builder.spawn((NodeBundle {
                        style: Style {
                            width: Val::Px(4.0),
                            height: Val::Px(16.0),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(COLOR_PITCH_GOOD),
                        ..Default::default()
                    }, CentsMarker));

                });
            });
        });
}

pub fn tune_update(
    back_button_query_interaction: Query<&Interaction, With<BackButton>>,
    continue_button_query_interaction: Query<&Interaction, With<ContinueButton>>,
    input_device: Res<InputDeviceResource>,
    mut next_state: ResMut<NextState<AppState>>,
    mut note_text_query: Query<&mut Text, With<NoteMarker>>,
    mut cents_marker_query: Query<(&mut Style, &mut BackgroundColor), With<CentsMarker>>,
) {
    if let Some(audio_stream) = &input_device.audio_stream_main {
        if let Some(pitch) = audio_stream.get_pitch() {
            let note = note_from_pitch(pitch);
            let note_string = NOTE_STRINGS[(note % 12) as usize];
            let cents_off = cents_off_from_pitch(pitch, note);
            let octave = octave_from_note(note);

            for mut text in note_text_query.iter_mut() {
                text.sections[0].value = format!("{}{}", note_string, octave);
            }
            for (mut style, mut background_color) in cents_marker_query.iter_mut() {
                style.margin = UiRect::left(Val::Px((cents_off * 4) as f32));
                if cents_off > -10 && cents_off < 10 {
                    *background_color = BackgroundColor(COLOR_PITCH_GOOD);
                } else {
                    *background_color = BackgroundColor(COLOR_PITCH_BAD);
                }
            }
        }
    }

    for interaction in back_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::InputDeviceDetail);
        }
    }

    for interaction in continue_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::SongSelect);
        }
    }
}

pub fn tune_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TuneMarker>>,
    mut input_device: ResMut<InputDeviceResource>,
) {
    input_device.audio_stream_main = None;

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

