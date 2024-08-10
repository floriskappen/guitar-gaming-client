use bevy::prelude::*;

use crate::{helpers::input_device::AudioStream, resources::{configuration::{Configuration, DeviceChannel}, input_device::InputDevice}};

#[derive(Component)]
pub struct AudioBarWrapper {
    pub channel: DeviceChannel,
}

#[derive(Component)]
pub struct AudioBar {
    pub channel: DeviceChannel,
}

pub fn spawn_audio_bar(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    channel: DeviceChannel
) {
    builder.spawn((NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(4.0)),
                height: Val::Px(300.0 + (8.0 * 2.0) + (4.0 * 2.0)),
                ..Default::default()
            },
            border_color: BorderColor(Color::srgb(0.92, 0.27, 0.09)),

            ..Default::default()
        }, AudioBarWrapper { channel: channel.clone() })).with_children(|builder| {
        builder.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(200.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            },
            AudioBar { channel }
        ));
    });
}

pub fn audio_bar_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_bar: Query<(&mut Style, &AudioBar), With<AudioBar>>,
    audio_bar_wrapper_interaction_query: Query<(&Interaction, &AudioBarWrapper), (Changed<Interaction>, With<Button>)>,
    mut configuration: ResMut<Configuration>,
    mut input_device: ResMut<InputDevice>,
) {
    for (mut style, audio_bar) in audio_bar.iter_mut() {
        if audio_bar.channel == DeviceChannel::One {
            if input_device.audio_stream_left.is_none() {
                input_device.audio_stream_left = Some(AudioStream::new(configuration.device.clone().unwrap(), DeviceChannel::One, 1024).unwrap())
            }
            if let Some(audio_stream_left) = &input_device.audio_stream_left {
                style.height = Val::Px((audio_stream_left.get_current_amplitude() * 200.0).min(300.0))
            }

        } else if audio_bar.channel == DeviceChannel::Two {
            if input_device.audio_stream_right.is_none() {
                input_device.audio_stream_right = Some(AudioStream::new(configuration.device.clone().unwrap(), DeviceChannel::Two, 1024).unwrap())
            }
            if let Some(audio_stream_right) = &input_device.audio_stream_right {
                style.height = Val::Px((audio_stream_right.get_current_amplitude() * 200.0).min(300.0))
            }
        }
    }

    for (interaction, button) in audio_bar_wrapper_interaction_query.iter() {
        if button.channel == DeviceChannel::One {
            if configuration.selected_device_channel.is_none() {
                configuration.selected_device_channel = Some(DeviceChannel::One)
            } else if configuration.selected_device_channel == Some(DeviceChannel::One) {
                configuration.selected_device_channel = None
            } else if configuration.selected_device_channel == Some(DeviceChannel::Two) {
                configuration.selected_device_channel = Some(DeviceChannel::All)
            } else if configuration.selected_device_channel == Some(DeviceChannel::All) {
                configuration.selected_device_channel = Some(DeviceChannel::Two)
            }
        } else if button.channel == DeviceChannel::Two {
            if configuration.selected_device_channel.is_none() {
                configuration.selected_device_channel = Some(DeviceChannel::Two)
            } else if configuration.selected_device_channel == Some(DeviceChannel::Two) {
                configuration.selected_device_channel = None
            } else if configuration.selected_device_channel == Some(DeviceChannel::One) {
                configuration.selected_device_channel = Some(DeviceChannel::All)
            } else if configuration.selected_device_channel == Some(DeviceChannel::All) {
                configuration.selected_device_channel = Some(DeviceChannel::One)
            }
        }
    }
}
