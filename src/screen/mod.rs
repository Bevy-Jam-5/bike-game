//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>().add_sub_state::<PlayState>();
    app.enable_state_scoped_entities::<Screen>();
    app.enable_state_scoped_entities::<PlayState>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
    ));

    app.add_systems(OnEnter(Screen::EnterPlaying), enter_playing);
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    // This is needed as an intermediary state to allow resetting the game world.
    EnterPlaying,
    Playing,
}

/// Sub-states for the [`Screen::Playing`] state.
#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[source(Screen = Screen::Playing)]

pub enum PlayState {
    #[default]
    Spawning,
    LoadingPipelines,
    Active,
    GameEnded,
}

fn enter_playing(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}
