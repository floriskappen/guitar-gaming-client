use bevy::prelude::*;

use crate::{components::button_minimal::spawn_button_minimal, helpers::song_library::scan_song_library, resources::{song_library::SongLibraryResource, song_loaded::SongLoadedResource}, states::app_state::AppState};

use super::song_list::{spawn_song_list, SongListItemMarker};

#[derive(Component)]
pub struct SongSelectMarker;
#[derive(Component)]
pub struct ChangeInputDeviceButton;
#[derive(Component)]
pub struct RefreshSongLibraryButton;
#[derive(Component)]
pub struct SongListWrapperMarker;

pub fn song_select_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    song_library: Res<SongLibraryResource>,
) {
    commands.spawn((Camera2dBundle::default(), SongSelectMarker));
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
        ..Default::default()
    }, SongSelectMarker))
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
                    "< change input device",
                    ChangeInputDeviceButton
                );
                // Refresh song library button
                spawn_button_minimal(
                    builder,
                    &asset_server,
                    "refresh song library",
                    RefreshSongLibraryButton
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
                    "select a song",
                    TextStyle {
                        font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    }
                ));


                builder.spawn((NodeBundle {
                    ..Default::default()
                }, SongListWrapperMarker))
                    .with_children(|builder| {
                        spawn_song_list(builder, &asset_server, &song_library.songs);
                    });
            });
        });
}

pub fn song_select_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    change_input_device_button_query_interaction: Query<&Interaction, With<ChangeInputDeviceButton>>,
    refresh_song_library_button_query_interaction: Query<&Interaction, With<RefreshSongLibraryButton>>,
    song_list_element_query_interaction: Query<(&SongListItemMarker, &Interaction), With<SongListItemMarker>>,
    song_list_wrapper_query: Query<Entity, With<SongListWrapperMarker>>,
    mut song_library: ResMut<SongLibraryResource>,
    mut song_loaded: ResMut<SongLoadedResource>,
    mut next_state: ResMut<NextState<AppState>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    for interaction in change_input_device_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::InputDeviceOverview);
        }
    }
    for interaction in refresh_song_library_button_query_interaction.iter() {
        if *interaction == Interaction::Pressed && buttons.just_pressed(MouseButton::Left) {
            song_library.songs = scan_song_library().unwrap();
            song_library.save_to_disk();

            for entity in song_list_wrapper_query.iter() {
                commands.entity(entity).despawn_descendants();
                commands.entity(entity).with_children(|builder| {
                    spawn_song_list(builder, &asset_server, &song_library.songs);
                });
            }
        }
    }
    for (song_list_element, interaction) in song_list_element_query_interaction.iter() {
        if *interaction == Interaction::Pressed && buttons.just_pressed(MouseButton::Left) {
            if let Some(selected_song) = song_library.find_by_id(&song_list_element.uuid) {
                song_loaded.load_song(selected_song.clone());
                next_state.set(AppState::SongPlay);
            }
        }
    }
}

pub fn song_select_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SongSelectMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
