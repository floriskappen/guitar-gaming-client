use bevy::prelude::*;
use cpal::traits::{HostTrait, DeviceTrait};

use crate::{components::button_primary::spawn_button, plugins::input_devices::SelectInputDeviceButton, resources::input_devices::InputDevices};

const MARGIN: Val = Val::Px(12.);

#[derive(Component)]
pub struct InputDeviceOverviewMarker;

pub fn state_input_device_overview(mut commands: Commands, asset_server: Res<AssetServer>, mut input_devices: ResMut<InputDevices>) {
    let devices = input_devices.host.input_devices().unwrap();
    let device_list = devices.collect();
    input_devices.input_devices = device_list;

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(MARGIN),
                    row_gap: MARGIN,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            InputDeviceOverviewMarker
        ))
        .with_children(|builder| {
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
                            device_name.clone(),
                            SelectInputDeviceButton
                        );
                    }
                });
        });
}

pub fn cleanup_input_device_overview(
    mut commands: Commands,
    query: Query<Entity, With<InputDeviceOverviewMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
