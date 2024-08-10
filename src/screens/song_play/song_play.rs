use bevy::prelude::*;

use crate::{components::button_minimal::spawn_button_minimal, resources::song_loaded::SongLoadedResource, states::app_state::AppState};


#[derive(Component)]
pub struct SongPlayMarker;
#[derive(Component)]
pub struct BackButtonMarker;

pub fn song_play_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    song_loaded: Res<SongLoadedResource>,
) {
    commands.spawn((Camera2dBundle::default(), SongPlayMarker));
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
        ..Default::default()
    }, SongPlayMarker))
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
                    "< back to song select",
                    BackButtonMarker
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
                let song_metadata = song_loaded.metadata.clone().unwrap();
                builder.spawn(TextBundle::from_section(
                    format!(
                        "{} - {}",
                        song_metadata.artists.join(", "),
                        song_metadata.title.clone()
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    }
                ));
            });
        });
}

pub fn song_play_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    back_button_query_interaction: Query<&Interaction, With<BackButtonMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in back_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::SongSelect);
        }
    }
}

pub fn song_play_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SongPlayMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
