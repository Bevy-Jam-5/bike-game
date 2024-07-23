use avian3d::prelude::Collider;
use bevy::prelude::*;
use bevy_tnua::controller::TnuaControllerBundle;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::*;

use crate::game::movement::{DesiredVelocity, LastPedal, PedalTimer, PlayerMovement};
use crate::third_party::leafwing_input_manager::PlayerAction;

use super::first_person_camera::SpawnFirstPersonCamera;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.observe(on_player_spawn);
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct Player;

fn on_player_spawn(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.entity()).insert((
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        LastPedal::default(),
        PedalTimer::default(),
        PlayerMovement::default(),
        TnuaControllerBundle::default(),
        TnuaAvian3dSensorShape(Collider::round_cuboid(0.3, 0.8, 1.55, 0.1)),
        DesiredVelocity::default(),
    ));

    commands.trigger(SpawnFirstPersonCamera);
}
