//! Spawn the HUD by triggering other observers.

use bevy::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_hud);
}

#[derive(Event, Debug)]
pub struct SpawnHud;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct TimeText;

fn spawn_hud(_trigger: Trigger<SpawnHud>, mut commands: Commands) {
    use Val::*;

    let text_style = TextStyle {
        font_size: 24.0,
        color: Color::WHITE,
        ..default()
    };

    commands
        .spawn((
            Name::new("HUD"),
            NodeBundle {
                background_color: Color::BLACK.with_alpha(0.5).into(),
                style: Style {
                    padding: UiRect::all(Px(10.0)),
                    top: Px(0.0),
                    left: Px(0.0),
                    row_gap: Px(10.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Money text"),
                MoneyText,
                TextBundle::from_sections([
                    TextSection::new("Money: ", text_style.clone()),
                    TextSection::new("$0", text_style.clone()),
                ]),
                StateScoped(Screen::Playing),
            ));
            children.spawn((
                Name::new("Time text"),
                TimeText,
                TextBundle::from_sections([
                    TextSection::new("Time: ", text_style.clone()),
                    TextSection::new("00:00", text_style),
                ]),
                StateScoped(Screen::Playing),
            ));
        });
}
