use avian3d::prelude::*;
use bevy::prelude::*;

pub use self::{
    collision_layer::CollisionLayerPreset, disable_collider::{DisableCollider, DisableColliderCommandsExt},
};

mod collision_layer;
mod disable_collider;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        collision_layer::plugin,
        disable_collider::plugin,
    ));
}
