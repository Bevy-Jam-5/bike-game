use bevy::prelude::*;

use super::{money::Money, time::InGameTime};
use crate::{
    game::time::format_duration_to_mm_ss,
    screen::{PlayState, Screen},
    ui::prelude::*,
};

// FIXME: Change this to a larger value once we have more quests.
pub const GAME_END_MONEY: f32 = 4.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GameEndAction>();

    app.add_systems(
        Update,
        (
            end_game,
            handle_game_end_action.run_if(in_state(PlayState::GameEnded)),
        ),
    );
    app.add_systems(OnEnter(PlayState::GameEnded), on_game_end);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum GameEndAction {
    PlayAgain,
    TitleScreen,
}

fn end_game(money: Res<Money>, mut next_state: ResMut<NextState<PlayState>>) {
    if !money.is_changed() {
        return;
    }

    if money.0 > GAME_END_MONEY {
        next_state.set(PlayState::GameEnded);
    }
}

fn on_game_end(mut commands: Commands, money: Res<Money>, time: Res<InGameTime>) {
    use Val::*;

    let header_text_style = TextStyle {
        font_size: 42.0,
        color: Color::WHITE,
        ..default()
    };

    let label_text_style = TextStyle {
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    };

    commands.ui_root().with_children(|children| {
        children
            .spawn((
                Name::new("Game end screen"),
                NodeBundle {
                    style: Style {
                        width: Percent(50.0),
                        padding: UiRect::all(Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Px(10.0),
                        ..default()
                    },
                    background_color: Color::BLACK.with_alpha(0.75).into(),
                    border_radius: BorderRadius::all(Px(10.0)),
                    ..default()
                },
                StateScoped(Screen::Playing),
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("Game end text"),
                    TextBundle {
                        style: Style {
                            margin: UiRect::bottom(Px(10.0)),
                            ..default()
                        },
                        ..TextBundle::from_section("Game ended", header_text_style)
                    },
                ));

                children.spawn((
                    Name::new("Money text"),
                    TextBundle::from_section(
                        format!("Earned: ${}", money.0),
                        label_text_style.clone(),
                    ),
                ));

                children.spawn((
                    Name::new("Time text"),
                    TextBundle::from_section(
                        format!("Elapsed time: {}", format_duration_to_mm_ss(time.elapsed())),
                        label_text_style,
                    ),
                ));

                children
                    .spawn(NodeBundle {
                        style: Style {
                            width: Percent(100.0),
                            margin: UiRect::top(Px(20.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|children| {
                        children
                            .button("Play again")
                            .insert(GameEndAction::PlayAgain);
                        children
                            .button("Return to title screen")
                            .insert(GameEndAction::TitleScreen);
                    });
            });
    });
}

fn handle_game_end_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&GameEndAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                GameEndAction::PlayAgain => next_screen.set(Screen::EnterPlaying),
                GameEndAction::TitleScreen => next_screen.set(Screen::Title),
            }
        }
    }
}
