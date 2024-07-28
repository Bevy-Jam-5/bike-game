//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::{PlayState, Screen};
use crate::game::{
    assets::SoundtrackKey,
    audio::soundtrack::PlaySoundtrack,
    spawn::{hud::SpawnHud, level::SpawnLevel},
};
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnEnter(PlayState::Active), enter_active);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnLevel);
    commands
        .ui_root()
        .insert(StateScoped(PlayState::Spawning))
        .with_children(|children| {
            children.label("Spawning Level...");
            children.label("");
            children.label("This takes a while.");
            children.label("No worries, nothing crashed :)");
        });
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn enter_active(mut commands: Commands) {
    commands.trigger(SpawnHud);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}
