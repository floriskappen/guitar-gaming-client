use bevy::prelude::*;

use crate::components::button_minimal::spawn_button_minimal;

#[derive(Component)]
pub struct TuneMarker;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct ContinueButton;

pub fn tune_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle::default(), TuneMarker));
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
        ..Default::default()
    }, TuneMarker))
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
                    "< go back",
                    BackButton
                );

                // Continue button
                spawn_button_minimal(
                    builder,
                    &asset_server,
                    "continue >",
                    ContinueButton
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
                    "confirm tuning",
                    TextStyle {
                        font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    }
                ));
            });
        });
}

pub fn tune_update() {}

pub fn tune_cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TuneMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

