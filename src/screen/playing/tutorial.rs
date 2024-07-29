use bevy::{prelude::*, ui::Val::*};
use ui_palette::DARK_BACKGROUND;

use super::PlayState;
use crate::game::assets::FontHandles;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Tutorial), enter_tutorial);
    app.add_systems(Update, exit_tutorial.run_if(in_state(PlayState::Tutorial)));
}

fn enter_tutorial(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: DARK_BACKGROUND.into(),
                ..default()
            },
            StateScoped(PlayState::Tutorial),
        ))
        .with_children(|children| {
            children
                .spawn((NodeBundle {
                    style: Style {
                        width: Px(750.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|children| {
                    let regular = TextStyle {
                        font: fonts.rubik_regular.clone_weak(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    };
                    let bold = TextStyle {
                        font: fonts.rubik_bold.clone_weak(),
                        font_size: 32.0,
                        color: Color::srgb(0.05, 0.95, 1.0),
                    };

                    children.spawn(TextBundle {
                        style: Style {
                            margin: UiRect::bottom(Px(10.0)),
                            ..default()
                        },
                        ..TextBundle::from_section(
                            "Instructions",
                            TextStyle {
                                font_size: 42.0,
                                ..bold.clone()
                            },
                        )
                    });

                    children.spawn(TextBundle::from_section(
                        "Your job is to help the citizens of Beville and perform deliveries for them. Collect enough money to win the game!",
                        regular.clone(),
                    ));

                    children.spawn(TextBundle::from_sections(vec![
                        TextSection::new("Alternate ", regular.clone()),
                        TextSection::new("A", bold.clone()),
                        TextSection::new(" and ", regular.clone()),
                        TextSection::new("D", bold.clone()),
                        TextSection::new(" to pedal.", regular.clone()),
                    ]));

                    children.spawn(TextBundle::from_sections(vec![
                        TextSection::new("Move the ", regular.clone()),
                        TextSection::new("mouse", bold.clone()),
                        TextSection::new(" to turn.", regular.clone()),
                    ]));


                    children.spawn(TextBundle::from_sections(vec![
                        TextSection::new("Collect money from doing ", regular.clone()),
                        TextSection::new("deliveries", bold.clone()),
                        TextSection::new(" or performing ", regular.clone()),
                        TextSection::new("stunts", bold.clone()),
                        TextSection::new(".", regular.clone()),
                    ]));

                    children.spawn(TextBundle {
                        style: Style {
                            margin: UiRect::top(Px(10.0)),
                            ..default()
                        },
                        ..TextBundle::from_section(
                            "Press any button to begin",
                            TextStyle {
                                font_size: 24.0,
                                ..regular.clone()
                            },
                        )
                    });
                });
        });
}

fn exit_tutorial(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<PlayState>>) {
    if input.get_just_pressed().next().is_some() {
        next_state.set(PlayState::Active);
    }
}
