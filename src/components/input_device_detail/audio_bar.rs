use bevy::prelude::*;

use crate::{helpers::input_device::AudioStream, resources::{configuration::{Configuration, DeviceChannel}, input_device::InputDevice}};

#[derive(Component)]
pub struct AudioBar {
    pub channel: DeviceChannel,
}

pub fn spawn_audio_bar(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    channel: DeviceChannel
) {
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
}

pub fn audio_bar_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Style, &AudioBar), With<AudioBar>>,
    interaction_query: Query<(&Interaction, &mut BackgroundColor, &AudioBar), (Changed<Interaction>, With<Button>)>,
    configuration: ResMut<Configuration>,
    mut input_device: ResMut<InputDevice>,
) {
    for (mut style, audio_bar) in query.iter_mut() {
        if audio_bar.channel == DeviceChannel::L {
            if input_device.audio_stream_left.is_none() {
                input_device.audio_stream_left = Some(AudioStream::new(configuration.device.clone().unwrap(), DeviceChannel::L, 1024).unwrap())
            }
            if let Some(audio_stream_left) = &input_device.audio_stream_left {
                style.height = Val::Px(audio_stream_left.get_current_amplitude() * 400.0)
            }

        } else if audio_bar.channel == DeviceChannel::R {
            if input_device.audio_stream_right.is_none() {
                input_device.audio_stream_right = Some(AudioStream::new(configuration.device.clone().unwrap(), DeviceChannel::R, 1024).unwrap())
            }
            if let Some(audio_stream_right) = &input_device.audio_stream_right {
                style.height = Val::Px(audio_stream_right.get_current_amplitude() * 400.0)
            }
        }

    }
}
