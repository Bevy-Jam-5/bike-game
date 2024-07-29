//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::{game::assets::FontHandles, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("Title text"),
                    NodeBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|children| {
                    children.spawn((
                        Name::new("Header Text"),
                        TextBundle::from_section(
                            "Crazy Bike",
                            TextStyle {
                                font: fonts.rubik_bold.clone_weak(),
                                font_size: 42.0,
                                color: Color::WHITE,
                            },
                        ),
                    ));
                });
            children
                .button("Play", fonts.rubik_regular.clone_weak())
                .insert(TitleAction::Play);
            children
                .button("Credits", fonts.rubik_regular.clone_weak())
                .insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children
                .button("Exit", fonts.rubik_regular.clone_weak())
                .insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::EnterPlaying),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
