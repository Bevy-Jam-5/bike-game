use bevy::prelude::*;
use bevy_tnua::builtins::TnuaBuiltinWalk;

use crate::util::single_mut;
use crate::{FixedAppSet};
use crate::{third_party::leafwing_input_manager::PlayerAction, util::single};
use avian3d::prelude::*;
use bevy_tnua::controller::TnuaController;
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

use super::spawn::first_person_camera::FirstPersonCamera;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(LastPedal, PedalTimer)>();
    app.add_event::<PedalEvent>();
    app.add_systems(
        PreUpdate,
        update_pedal.after(InputManagerSystem::ManualControl),
    );
    app.add_systems(
        FixedUpdate,
        (on_pedal, turn, dampen_movement)
            .chain()
            .in_set(FixedAppSet::ControllerMovement),
    );
    app.add_systems(Update, apply_movement_basis);
}

#[derive(Debug, Component, Clone, Reflect)]
#[reflect(Debug, Component, Default)]
pub struct PlayerMovement {
    /// Damping coefficient for movement when grounded.
    pub ground_damping: f32,
    /// How much velocity one pedal adds.
    pub pedal_acceleration: f32,
    /// The maximum speed that can be reached by pedalling in meters per second.
    pub max_pedal_speed: f32,
    /// How quickly the bike turns in radians per second when moving at 1 m/s.
    /// Scales based on linear velocity.
    pub turn_speed: f32,
    /// The maximum turning speed that the bike can reach in radians per second.
    pub max_turn_speed: f32,
    /// How sensitive turning controls are.
    pub turn_sensitivity: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            ground_damping: 0.3,
            pedal_acceleration: 2.0,
            max_pedal_speed: 10.0,
            turn_speed: 0.8,
            max_turn_speed: 3.0,
            turn_sensitivity: 0.01,
        }
    }
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq, Default)]
#[reflect(Debug, Component, PartialEq, Default)]
pub enum LastPedal {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Debug, Component, Clone, Reflect, Deref, DerefMut, Default)]
#[reflect(Debug, Component, Default)]
pub struct PedalTimer(Timer);

#[derive(Debug, Event, Clone, Reflect, Default)]
#[reflect(Debug, Default)]
pub struct PedalEvent;

fn update_pedal(
    time: Res<Time>,
    mut q_player: Query<(&ActionState<PlayerAction>, &mut PedalTimer, &mut LastPedal)>,
    mut pedal_events: EventWriter<PedalEvent>,
) {
    let (action, mut timer, mut last_pedal) = single_mut!(q_player);
    timer.tick(time.delta());
    if timer.finished() {
        *last_pedal = LastPedal::None;
    }
    const PEDAL_SECS: f32 = 1.0;

    let pedalled = if action.just_pressed(&PlayerAction::PedalLeft)
        && matches!(*last_pedal, LastPedal::Right | LastPedal::None)
    {
        *last_pedal = LastPedal::Left;
        true
    } else if action.just_pressed(&PlayerAction::PedalRight)
        && matches!(*last_pedal, LastPedal::Left | LastPedal::None)
    {
        *last_pedal = LastPedal::Right;
        true
    } else {
        false
    };

    if !pedalled {
        return;
    }

    timer.0 = Timer::from_seconds(PEDAL_SECS, TimerMode::Once);
    pedal_events.send(PedalEvent);
}

fn on_pedal(
    mut pedal_events: EventReader<PedalEvent>,
    mut q_player: Query<(&GlobalTransform, &PlayerMovement, &mut LinearVelocity)>,
) {
    let (transform, movement, mut lin_vel) = single_mut!(q_player);
    for _ in pedal_events.read() {
        if lin_vel.length_squared() < movement.max_pedal_speed.powi(2) {
            lin_vel.0 += movement.pedal_acceleration * transform.forward();
        }
    }
}

fn turn(
    mut q_player: Query<(&mut LinearVelocity, &PlayerMovement)>,
    q_camera: Query<&GlobalTransform, With<FirstPersonCamera>>,
    time: Res<Time>,
) {
    let camera_transform = single!(q_camera);
    let delta_seconds = time.delta_seconds();
    let (mut lin_vel, movement) = single_mut!(q_player);
    let rcp = lin_vel.0.length_recip();
    if rcp.is_infinite() || rcp == 0.0 {
        return;
    }

    let horizontal = camera_transform.forward().with_y(0.0).normalize();
    let rotation = Quat::from_rotation_arc(lin_vel.0.normalize(), horizontal);
    lin_vel.0 = rotation * lin_vel.0;
}

fn dampen_movement(
    mut query: Query<(&mut LinearVelocity, &PlayerMovement, &TnuaController)>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut lin_vel, movement, controller) in &mut query {
        if !controller.is_airborne().unwrap_or(false) {
            let damping = 1.0 / (1.0 + delta_seconds * movement.ground_damping);
            lin_vel.x *= damping;
            lin_vel.z *= damping;
        }
    }
}

fn apply_movement_basis(mut query: Query<(&mut TnuaController, &LinearVelocity)>) {
    for (mut controller, lin_vel) in &mut query {
        controller.basis(TnuaBuiltinWalk {
            desired_forward: lin_vel.normalize_or_zero(),
            desired_velocity: lin_vel.0,
            float_height: 1.25,
            ..default()
        });
    }
}
