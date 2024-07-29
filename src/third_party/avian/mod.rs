use avian3d::prelude::*;
use bevy::prelude::*;

pub use self::{
    collision_layer::CollisionLayerPreset,
    disable_sensor::{DisableSensor, DisableSensorCommandsExt},
};

mod collision_layer;
mod disable_sensor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        collision_layer::plugin,
        disable_sensor::plugin,
    ));
}
