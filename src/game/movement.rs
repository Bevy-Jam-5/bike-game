use bevy::prelude::*;
use bevy_tnua::builtins::TnuaBuiltinWalk;

use crate::screen::PlayState;
use crate::util::single_mut;
use crate::FixedAppSet;
use crate::{third_party::leafwing_input_manager::PlayerAction, util::single};
use bevy_tnua::controller::TnuaController;
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

use super::spawn::{first_person_camera::FirstPersonCamera, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(LastPedal, PedalTimer, DesiredVelocity)>();
    app.add_event::<PedalEvent>();
    app.add_systems(
        PreUpdate,
        update_pedal.after(InputManagerSystem::ManualControl),
    );
    app.add_systems(
        FixedUpdate,
        (
            on_pedal.run_if(in_state(PlayState::Active)),
            turn,
            dampen_movement,
        )
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
    /// How quickly the bike turns in radians per second when moving at 1 m/s and looking fully sideways.
    /// Scales based on linear velocity.
    pub turn_speed: f32,
    /// The maximum turning speed that the bike can reach in radians per second.
    pub max_turn_speed: f32,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Reflect, Default, Deref, DerefMut)]
#[reflect(Debug, Component, Default, PartialEq)]
pub struct DesiredVelocity(pub Vec3);

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            ground_damping: 0.6,
            pedal_acceleration: 2.0,
            max_pedal_speed: 16.0,
            turn_speed: 0.8,
            max_turn_speed: 3.0,
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
    mut q_player: Query<(&GlobalTransform, &PlayerMovement, &mut DesiredVelocity)>,
) {
    let (transform, movement, mut lin_vel) = single_mut!(q_player);
    for _ in pedal_events.read() {
        if lin_vel.length_squared() < movement.max_pedal_speed.powi(2) {
            lin_vel.0 += movement.pedal_acceleration * transform.forward();
        }
    }
}

fn turn(
    time: Res<Time>,
    mut q_player: Query<(&mut DesiredVelocity, &PlayerMovement), With<Player>>,
    q_camera: Query<&GlobalTransform, With<FirstPersonCamera>>,
) {
    let dt = time.delta_seconds();
    let camera_transform = single!(q_camera);
    let (mut lin_vel, movement) = single_mut!(q_player);

    let origin = lin_vel.xz();
    if origin.is_zero() {
        return;
    }
    let target = camera_transform.forward().xz();
    if target.is_zero() {
        return;
    }

    // In range `[-π, +π]`. Note that doing this for Vec3 would only return positive values.
    let angle = origin.angle_between(target);
    // Looking sideways corresponds to an angle of π/2, so we divide by that to normalize.
    let horizontal = angle * std::f32::consts::FRAC_2_PI;

    let rotation = (movement.turn_speed * lin_vel.length() * -horizontal)
        .clamp(-movement.max_turn_speed, movement.max_turn_speed)
        * dt;
    let rotation = Quat::from_rotation_y(rotation);

    lin_vel.0 = rotation * lin_vel.0;
}

fn dampen_movement(
    mut query: Query<(&mut DesiredVelocity, &PlayerMovement, &TnuaController)>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut lin_vel, movement, controller) in &mut query {
        if !controller.is_airborne().unwrap_or(false) {
            let damping = 1.0 / (1.0 + delta_seconds * movement.ground_damping);
            lin_vel.x *= damping;
            lin_vel.y = 0.0;
            lin_vel.z *= damping;
        }
    }
}

fn apply_movement_basis(mut query: Query<(&mut TnuaController, &DesiredVelocity)>) {
    for (mut controller, lin_vel) in &mut query {
        controller.basis(TnuaBuiltinWalk {
            desired_forward: lin_vel.normalize_or_zero(),
            desired_velocity: lin_vel.0,
            float_height: 1.0,
            cling_distance: 0.02,
            max_slope: std::f32::consts::FRAC_PI_8,
            free_fall_extra_gravity: 0.0,
            turning_angvel: 3.0,
            ..default()
        });
    }
}

trait Vec3Ext: Copy {
    fn is_zero(self) -> bool;
}

impl Vec3Ext for Vec2 {
    fn is_zero(self) -> bool {
        let len_sq = self.length_squared();
        len_sq == 0.0 || !len_sq.is_finite()
    }
}
