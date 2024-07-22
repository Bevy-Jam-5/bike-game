//! Game mechanics and content.

use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod camera;
pub mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        movement::plugin,
        camera::plugin,
    ));
}
