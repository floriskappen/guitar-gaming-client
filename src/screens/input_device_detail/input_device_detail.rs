use bevy::prelude::*;

use crate::{components::button_minimal::spawn_button_minimal, resources::{configuration::ConfigurationResource, input_device::InputDeviceResource}, screens::input_device_detail::audio_bar::spawn_audio_bar, states::app_state::AppState};

#[derive(Component)]
pub struct BackButton;
#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct InputDeviceDetailMarker;

pub fn input_device_detail_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut input_device: ResMut<InputDeviceResource>,
    mut configuration: ResMut<ConfigurationResource>,
) {
    // Get amount of channels in selected device
    let device_configuration = input_device.configuration.clone().unwrap();
    let channels = device_configuration.channels();

    // Create an audio stream for each channel
    let mut audio_stream_channels = vec![];
    for _ in 0..device_configuration.channels() {
        audio_stream_channels.push(None);
    }
    input_device.audio_stream_channels = Some(audio_stream_channels);

    // Generate UI
    commands.spawn((Camera2dBundle::default(), InputDeviceDetailMarker));
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
        ..Default::default()
    }, InputDeviceDetailMarker))
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
                        "select which audio channels to use",
                        TextStyle {
                            font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                            font_size: 18.0,
                            color: Color::WHITE,
                        }
                    ));

                    // Audio bars wrapper
                    builder.spawn(
                        NodeBundle {
                            style: Style {
                                // fill the entire window
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(12.)),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                            ..Default::default()
                        },
                    )
                        .with_children(|builder| {
                            // Audio bars
                            for channel in 0..channels {
                                let is_selected_initially = configuration.selected_device_channels.contains(&channel);
                                spawn_audio_bar(builder, &asset_server, channel, is_selected_initially);
                            }
                        });
                });
        });

}

pub fn input_device_detail_update(
    back_button_query_interaction: Query<&Interaction, With<BackButton>>,
    continue_button_query_interaction: Query<&Interaction, With<ContinueButton>>,
    continue_button_query: Query<&Children, With<ContinueButton>>,
    mut continue_button_query_children: Query<&mut Text>,
    mut next_state: ResMut<NextState<AppState>>,
    configuration: Res<ConfigurationResource>,
) {
    for interaction in back_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::InputDeviceOverview);
        }
    }

    let can_continue = !configuration.selected_device_channels.is_empty();

    for interaction in continue_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed && can_continue {
            next_state.set(AppState::Tune);
        }
    }

    for children in continue_button_query.iter() {
        for &child in children.iter() {
            if let Ok(mut text) = continue_button_query_children.get_mut(child) {
                if !can_continue {
                    text.sections[0].style.color = Color::srgba(1.0, 1.0, 1.0, 0.2)
                } else {
                    text.sections[0].style.color = Color::srgba(1.0, 1.0, 1.0, 1.0)
                }
            }
        }
    }
}

pub fn input_device_detail_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<InputDeviceDetailMarker>>,
    mut input_device: ResMut<InputDeviceResource>,
) {
    input_device.audio_stream_channels = None;
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
