use avian3d::prelude::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::*;

use crate::game::movement::{DesiredVelocity, LastPedal, PedalTimer, PlayerMovement};
use crate::third_party::{avian::CollisionLayerPreset, leafwing_input_manager::PlayerAction};

use super::first_person_camera::SpawnFirstPersonCamera;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
}

#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct Player;

impl Component for Player {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let mut commands = world.commands();
            let collider = Collider::capsule(0.25, 1.0);
            commands.entity(entity).insert((
                InputManagerBundle::with_map(PlayerAction::default_input_map()),
                LastPedal::default(),
                PedalTimer::default(),
                PlayerMovement::default(),
                TnuaControllerBundle::default(),
                TnuaAvian3dSensorShape(collider.clone()),
                DesiredVelocity::default(),
                CollisionLayerPreset::Player,
                collider,
                RigidBody::Dynamic,
                ColliderDensity(300.0),
            ));
            commands.trigger(SpawnFirstPersonCamera);
        });
    }
}
