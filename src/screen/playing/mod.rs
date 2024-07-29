//! The screen state for the main game loop.

#[cfg(feature = "dev")]
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use super::{PlayState, Screen};
use crate::game::audio::soundtrack::PlaySoundtrack;

pub mod active;
pub mod loading_pipelines;
pub mod spawning;
pub mod tutorial;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    #[cfg(feature = "dev")]
    {
        app.add_systems(
            Update,
            return_to_title_screen
                .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
        );
    }
    app.add_plugins((
        spawning::plugin,
        loading_pipelines::plugin,
        tutorial::plugin,
        active::plugin,
    ));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

#[cfg(feature = "dev")]
fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Debug, Component)]
struct LoadingText;
