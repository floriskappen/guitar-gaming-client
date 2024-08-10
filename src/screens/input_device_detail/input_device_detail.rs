use bevy::prelude::*;
use cpal::traits::DeviceTrait;

use crate::{resources::configuration::{Configuration, DeviceChannel}, screens::input_device_detail::audio_bar::spawn_audio_bar};

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
    let channels = config.channels();

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
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(12.)),
                    row_gap: Val::Px(12.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                ..Default::default()
            },
            InputDeviceDetailMarker
        ))
        .with_children(|builder| {
            if channels > 1 {
                spawn_audio_bar(builder, &asset_server, DeviceChannel::One);
                spawn_audio_bar(builder, &asset_server, DeviceChannel::Two);
            } else {
                spawn_audio_bar(builder, &asset_server, DeviceChannel::One);
            }
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
