use bevy::prelude::*;

use crate::{helpers::input_device::AudioStream, resources::{configuration::ConfigurationResource, input_device::InputDeviceResource}};

#[derive(Component)]
pub struct AudioBarCheckIcon {
    pub channel: u16,
}

#[derive(Component)]
pub struct AudioBarWrapper {
    pub channel: u16,
}

#[derive(Component)]
pub struct AudioBar {
    pub channel: u16,
}

const BORDER_COLOR_SELECTED: Color = Color::srgba(0.92, 0.27, 0.09, 1.0);
const BORDER_COLOR_UNSELECTED: Color = Color::srgba(0.92, 0.27, 0.09, 0.1);

pub fn spawn_audio_bar(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    channel: u16
) {

    builder.spawn((
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                margin: UiRect::horizontal(Val::Px(12.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        AudioBarWrapper { channel },
    )).with_children(|builder| {
        builder.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::bottom(Val::Px(8.0)),
                    ..Default::default()
                },
                ..Default::default()
            }
        ).with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                format!("ch {}", channel),
                TextStyle {
                    font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                }
            ));
        });

        // Audio bar
        builder.spawn(
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(8.0)),
                    border: UiRect::all(Val::Px(4.0)),
                    height: Val::Px(300.0 + (8.0 * 2.0) + (4.0 * 2.0)),
                    ..Default::default()
                },
                border_color: BorderColor(BORDER_COLOR_SELECTED),
    
                ..Default::default()
            },
        ).with_children(|builder| {
                builder.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(40.0),
                            height: Val::Px(0.0),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    },
                    AudioBar { channel }
                ));
        });

        // Check box
        builder.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    border: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::top(Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                border_color: BorderColor(BORDER_COLOR_SELECTED),
                ..Default::default()
            },
        )).with_children(|builder| {
            let icon = asset_server.load("icons/check.png");
            builder.spawn((ImageBundle {
                image: UiImage::new(icon),
                style: Style {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    ..Default::default()
                },
                ..Default::default()
            }, AudioBarCheckIcon { channel }));
        });
    });

}

pub fn audio_bar_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_bar: Query<(&mut Style, &AudioBar), With<AudioBar>>,
    mut audio_bar_check_icon: Query<(&mut Visibility, &AudioBarCheckIcon), With<AudioBarCheckIcon>>,
    mut audio_bar_wrapper_interaction_query: Query<(&Interaction, &AudioBarWrapper, &Children), (Changed<Interaction>, With<Button>)>,
    mut children_query: Query<&mut BorderColor>,
    mut configuration: ResMut<ConfigurationResource>,
    mut input_device: ResMut<InputDeviceResource>,
) {
    for (mut style, audio_bar) in audio_bar.iter_mut() {
        let device_configuration = input_device.configuration.clone().unwrap();
        
        if let Some(audio_stream_channels) = &mut input_device.audio_stream_channels {
            let audio_bar_channel_usize = audio_bar.channel as usize;

            if audio_stream_channels[audio_bar_channel_usize].is_none() {
                audio_stream_channels[audio_bar_channel_usize] = Some(AudioStream::new(configuration.device.clone().unwrap(), audio_bar.channel, 1024).unwrap())
            }

            if let Some(audio_stream) = &mut audio_stream_channels[audio_bar_channel_usize] {
                style.height = Val::Px((audio_stream.get_current_amplitude() * 200.0).min(300.0))
            }
        }
    }

    for (interaction, button, children) in audio_bar_wrapper_interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if configuration.selected_device_channels.contains(&button.channel) {
                // Remove from the vec
                configuration.selected_device_channels = configuration.selected_device_channels.clone().into_iter().filter(|&ch| ch != button.channel).collect::<Vec<u16>>();

                for &child in children.iter() {
                    if let Ok(mut border_color) = children_query.get_mut(child) {
                        *border_color = BorderColor(BORDER_COLOR_UNSELECTED);
                    }
                }
                for (mut visibility, audio_bar_check_icon) in audio_bar_check_icon.iter_mut() {
                    if audio_bar_check_icon.channel == button.channel {
                        *visibility = Visibility::Hidden;
                    }
                }
            } else {
                configuration.selected_device_channels.push(button.channel);
                for &child in children.iter() {
                    if let Ok(mut border_color) = children_query.get_mut(child) {
                        *border_color = BorderColor(BORDER_COLOR_SELECTED);
                    }
                }

                for (mut visibility, audio_bar_check_icon) in audio_bar_check_icon.iter_mut() {
                    if audio_bar_check_icon.channel == button.channel {
                        *visibility = Visibility::Visible;
                    }
                }
            }
        }
    }
}
