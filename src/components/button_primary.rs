use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonPrimary<T: 'static + Send + Sync> {
    pub data: T,
}

#[derive(Component)]
pub struct ButtonPrimaryMarker;

// Function to create a button entity
pub fn spawn_button<T: 'static + Send + Sync>(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
    color: Color,
    data: T,
    marker: impl Component + Default,
) {
    builder.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(color.into()),
            ..Default::default()
        },
        ButtonPrimary { data },
        marker, // Add the marker component to distinguish the button type
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/InterVariable.ttf"),
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
            *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35).into());
        }
        Interaction::None => {
            *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into());
        }
        _ => {}  // Ignore other interactions
    }
}