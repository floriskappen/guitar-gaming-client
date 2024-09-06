use bevy::prelude::*;

use crate::resources::song_library::SongMetadata;

#[derive(Component)]
pub struct SongListItemMarker {
    pub uuid: String
}

pub fn spawn_song_list(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    song_metadata_list: &Vec<SongMetadata>,
) {
    builder.spawn(
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
    .with_children(|builder| {
        if !song_metadata_list.is_empty() {
            for song_metadata in song_metadata_list {
                builder.spawn((ButtonBundle {
                    style: Style {
                        padding: UiRect { left: Val::Px(12.0), right: Val::Px(12.0), top: Val::Px(8.0), bottom: Val::Px(12.0) },
                        width: Val::Px(350.0),
                        border: UiRect::all(Val::Px(4.0)),
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::srgb(0.35, 0.35, 0.35)),
                    ..Default::default()
                }, SongListItemMarker { uuid: song_metadata.uuid.clone() })).with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        format!(
                            "{} - {}",
                            song_metadata.artists.join(", "),
                            song_metadata.title.clone()
                        ),
                        TextStyle {
                            font: asset_server.load("fonts/IBMPlexMono-Medium.ttf"),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            }
        } else {
            builder.spawn(TextBundle::from_section(
                "no songs found",
                TextStyle {
                    font: asset_server.load("fonts/IBMPlexMono-Medium.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ));
        }
    });
}
