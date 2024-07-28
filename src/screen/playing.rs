//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use blenvy::GameWorldTag;

use super::{PlayState, Screen};
use crate::ui::prelude::*;
use crate::{
    game::{
        assets::SoundtrackKey,
        audio::soundtrack::PlaySoundtrack,
        spawn::{hud::SpawnHud, level::SpawnLevel},
    },
    util::single_mut,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Spawning), enter_spawning);
    app.add_systems(
        OnEnter(PlayState::LoadingPipelines),
        enter_loading_pipelines,
    );
    app.add_systems(OnEnter(PlayState::Active), enter_active);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_spawning(mut commands: Commands) {
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

fn enter_loading_pipelines(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(PlayState::LoadingPipelines))
        .with_children(|children| {
            children.label("Loading graphics pipelines...");
        });
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn enter_active(mut commands: Commands, mut q_world: Query<&mut Visibility, With<GameWorldTag>>) {
    commands.trigger(SpawnHud);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    let mut visibility = single_mut!(q_world);
    *visibility = Visibility::Inherited;
}
