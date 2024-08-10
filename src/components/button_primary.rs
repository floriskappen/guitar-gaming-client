use bevy::prelude::*;

// Function to create a button entity
pub fn spawn_button(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    color: Color,
    marker: impl Component,
) {
    builder.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                padding: UiRect { left: Val::Px(12.0), right: Val::Px(12.0), top: Val::Px(8.0), bottom: Val::Px(12.0) },
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(color),
            ..Default::default()
        },
        marker, // Add the marker component to distinguish the button type
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/IBMPlexMono-Regular.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ));
    });
}

pub fn handle_generic_interaction(
    interaction: &Interaction,
    color: &mut BackgroundColor,
) {
    match *interaction {
        Interaction::Hovered => {
            *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
        }
        Interaction::None => {
            *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
        }
        _ => {}  // Ignore other interactions
    }
}