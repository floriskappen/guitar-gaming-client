use bevy::prelude::*;
use cpal::traits::{HostTrait, DeviceTrait};

use crate::{components::button_primary::{handle_generic_interaction, spawn_button}, resources::{configuration::ConfigurationResource, input_device::InputDeviceResource, input_devices::InputDevicesResource}, states::app_state::AppState};

#[derive(Component, Default)]
pub struct SelectInputDeviceButton {
    device_name: String
}

#[derive(Component)]
pub struct InputDeviceOverviewMarker;

pub fn input_device_overview_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut input_devices: ResMut<InputDevicesResource>
) {
    let devices = input_devices.host.input_devices().unwrap();
    let device_list = devices.collect();
    input_devices.input_devices = device_list;

    commands.spawn((Camera2dBundle::default(), InputDeviceOverviewMarker));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                ..Default::default()
            },
            InputDeviceOverviewMarker
        ))
        .with_children(|builder| {
            // Header (empty for now0)
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
            });

            // Content
            builder.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(24.),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "select an input device",
                    TextStyle {
                        font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    }
                ));

                // spawn the key
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        for device in &input_devices.input_devices {
                            let device_name = device.name().unwrap();
                            spawn_button(
                                builder,
                                &asset_server,
                                &device_name,
                                Color::srgb(0.15, 0.75, 0.15),
                                SelectInputDeviceButton { device_name: device_name.clone() }
                            );
                        }
                    });
            });

        });
}

pub fn input_device_overview_update(
    mut device_button_query_interaction: Query<(&Interaction, &mut BackgroundColor, &SelectInputDeviceButton), (Changed<Interaction>, With<SelectInputDeviceButton>)>,
    input_devices: Res<InputDevicesResource>,
    mut input_device: ResMut<InputDeviceResource>,
    mut configuration: ResMut<ConfigurationResource>,
    mut next_state: ResMut<NextState<AppState>>
) {
    input_device.audio_stream_channels = None;

    // Handle select device button interaction
    for (interaction, mut color, marker) in &mut device_button_query_interaction.iter_mut() {
        if *interaction == Interaction::Pressed {
            let device = input_devices.input_devices.iter().find(|&device| device.name().unwrap() == marker.device_name).unwrap().clone();
            let supported_configs = device.supported_input_configs().unwrap();
            let config = supported_configs
                .map(|config| config.with_max_sample_rate())
                .find(|config| config.channels() >= 1)
                .expect("No suitable configuration found");

            let channels = config.channels();

            configuration.device = Some(device);
            let mut initial_selected_chanels = vec![];
            for channel in 0..channels {
                initial_selected_chanels.push(channel);
            }
            configuration.selected_device_channels = initial_selected_chanels;

            next_state.set(AppState::InputDeviceDetail);
        } else {
            handle_generic_interaction(interaction, &mut color)
        }
    }
}

pub fn input_device_overview_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<InputDeviceOverviewMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
