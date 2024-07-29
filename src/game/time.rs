use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    screen::{PlayState, Screen},
    util::single_mut,
};

use super::spawn::hud::TimeText;

pub fn plugin(app: &mut App) {
    app.init_resource::<InGameTime>()
        .init_resource::<RemainingTime>()
        .register_type::<(InGameTime, RemainingTime)>();

    app.add_systems(
        Update,
        (
            tick_time,
            update_time_text.run_if(resource_changed::<InGameTime>),
        )
            .chain()
            .run_if(in_state(PlayState::Active)),
    );

    // Leaving the gameplay screen currently resets the world, so reset the timer.
    app.add_systems(OnExit(Screen::Playing), reset_time);
}

/// How much time the player has at the start of the game.
pub const INITIAL_TIME: Duration = Duration::from_secs(60);

#[derive(Debug, Resource, Clone, Default, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct InGameTime(pub Stopwatch);

#[derive(Debug, Resource, Clone, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct RemainingTime(pub Duration);

impl Default for RemainingTime {
    fn default() -> Self {
        Self(INITIAL_TIME)
    }
}

fn update_time_text(time: Res<RemainingTime>, mut time_text: Query<&mut Text, With<TimeText>>) {
    let mut text = single_mut!(time_text);
    text.sections[1].value = format_duration_to_mm_ss(time.0);
}

fn tick_time(
    mut in_game_time: ResMut<InGameTime>,
    mut remaining_time: ResMut<RemainingTime>,
    time: Res<Time>,
) {
    let delta = time.delta();
    in_game_time.tick(delta);
    remaining_time.0 = remaining_time.saturating_sub(delta);
}

fn reset_time(mut in_game_time: ResMut<InGameTime>, mut remaining_time: ResMut<RemainingTime>) {
    in_game_time.reset();
    *remaining_time = RemainingTime::default();
}

/// Returns a time string in the MM:SS format for the given duration.
pub fn format_duration_to_mm_ss(duration: Duration) -> String {
    let elapsed = duration.as_secs();
    let seconds = elapsed % 60;
    let minutes = (elapsed / 60) % 60;
    format!("{:0>2}:{:0>2}", minutes, seconds)
}
