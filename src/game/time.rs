use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{
    screen::{PlayState, Screen},
    util::single_mut,
};

use super::spawn::hud::TimeText;

pub fn plugin(app: &mut App) {
    app.init_resource::<InGameTime>()
        .register_type::<InGameTime>();

    app.add_systems(
        Update,
        (tick_in_game_time, update_time_text)
            .chain()
            .run_if(in_state(PlayState::Active)),
    );

    // Leaving the gameplay screen currently resets the world, so reset the timer.
    app.add_systems(OnExit(Screen::Playing), reset_in_game_time);
}

#[derive(Debug, Resource, Clone, Default, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct InGameTime(pub Stopwatch);

fn update_time_text(time: Res<InGameTime>, mut time_text: Query<&mut Text, With<TimeText>>) {
    if !time.is_changed() {
        return;
    }

    let mut text = single_mut!(time_text);
    text.sections[1].value = format_duration_to_mm_ss(time.elapsed());
}

fn tick_in_game_time(mut in_game_time: ResMut<InGameTime>, time: Res<Time>) {
    in_game_time.tick(time.delta());
}

fn reset_in_game_time(mut in_game_time: ResMut<InGameTime>) {
    in_game_time.reset();
}

/// Returns a time string in the MM:SS format for the given duration.
pub fn format_duration_to_mm_ss(duration: Duration) -> String {
    let elapsed = duration.as_secs();
    let seconds = elapsed % 60;
    let minutes = (elapsed / 60) % 60;
    format!("{:0>2}:{:0>2}", minutes, seconds)
}
