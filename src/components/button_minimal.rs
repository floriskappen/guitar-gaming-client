use bevy::prelude::*;

pub fn spawn_button_minimal(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    marker: impl Component,
) {
    builder.spawn((
        ButtonBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect { left: Val::Px(12.0), right: Val::Px(12.0), top: Val::Px(8.0), bottom: Val::Px(8.0) },
                ..Default::default()
            },
            ..Default::default()
        }, marker)
    ).with_children(|builder| {
        builder.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/IBMPlexMono-Medium.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
        ));
    });
}
