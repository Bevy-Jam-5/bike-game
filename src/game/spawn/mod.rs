//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod first_person_camera;
pub mod hud;
pub mod level;
pub mod player;
pub mod ui_camera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        player::plugin,
        ui_camera::plugin,
        hud::plugin,
        first_person_camera::plugin,
    ));
}
