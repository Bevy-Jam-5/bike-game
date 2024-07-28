use bevy::prelude::*;
use bevy_pipelines_ready::{PipelinesReady, PipelinesReadyPlugin};

use crate::screen::PlayState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PipelinesReadyPlugin);
    app.add_systems(Update, print.run_if(resource_changed::<PipelinesReady>));
    app.add_systems(
        Update,
        transition.run_if(in_state(PlayState::LoadingPipelines)),
    );
}
// This value should be found experimentally by logging `PipelinesReady` in your app
// during normal use and noting the maximum value.
#[cfg(not(target_arch = "wasm32"))]
// Sometimes 28, idk.
pub const EXPECTED_PIPELINES: usize = 25;
// The value will likely differ on the web due to different implementations of some
// render features.
#[cfg(target_arch = "wasm32")]
// Sometimes 21, idk.
pub const EXPECTED_PIPELINES: usize = 18;

fn print(ready: Res<PipelinesReady>) {
    info!("Pipelines Ready: {}/{}", ready.get(), EXPECTED_PIPELINES);
}

fn transition(ready: Res<PipelinesReady>, mut next_state: ResMut<NextState<PlayState>>) {
    if ready.get() >= EXPECTED_PIPELINES {
        next_state.set(PlayState::Active);
    }
}
