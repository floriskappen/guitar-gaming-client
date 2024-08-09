use bevy::prelude::*;
use cpal::traits::DeviceTrait;

use crate::{components::input_device_detail::audio_bar::spawn_audio_bar, resources::configuration::{Configuration, DeviceChannel}};

#[derive(Component)]
pub struct InputDeviceDetailMarker;

pub fn state_input_device_detail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    configuration: Res<Configuration>,
) {
    let selected_device = configuration.device.clone().unwrap();
    let supported_configs = selected_device.supported_input_configs().unwrap();

    let config = supported_configs
        .map(|config| config.with_max_sample_rate())
        .find(|config| config.channels() >= 1)
        .expect("No suitable configuration found");

    commands.spawn((Camera2dBundle::default(), InputDeviceDetailMarker));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // fill the entire window
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(12.)),
                    row_gap: Val::Px(12.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            InputDeviceDetailMarker
        ))
        .with_children(|builder| {
            spawn_audio_bar(builder, &asset_server, DeviceChannel::L);
            spawn_audio_bar(builder, &asset_server, DeviceChannel::R);
        });

    println!("{:?}", config);
}


pub fn cleanup_input_device_detail(
    mut commands: Commands,
    query: Query<Entity, With<InputDeviceDetailMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
