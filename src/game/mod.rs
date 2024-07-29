//! Game mechanics and content.

use bevy::prelude::*;
use materials::SingleColorMaterial;

pub mod assets;
pub mod audio;
pub mod camera;
pub mod fov_speed;
pub mod game_end;
pub mod materials;
pub mod money;
pub mod movement;
pub mod particle_emitter;
pub mod prop_yeet;
pub mod quest;
pub mod spawn;
pub mod stunt;
pub mod time;
pub mod view_model;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        movement::plugin,
        camera::plugin,
        quest::plugin,
        view_model::plugin,
        money::plugin,
        time::plugin,
        game_end::plugin,
        prop_yeet::plugin,
        fov_speed::plugin,
        particle_emitter::plugin::<Extrusion<Annulus>, SingleColorMaterial>,
        materials::plugin,
        stunt::plugin,
    ));
}
