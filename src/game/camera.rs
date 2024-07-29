use crate::screen::PlayState;
use crate::util::{single, single_mut};
use crate::{third_party::leafwing_input_manager::CameraAction, FixedAppSet};
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::spawn::{first_person_camera::FirstPersonCamera, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            rotate_camera.run_if(in_state(PlayState::Active)),
            clamp_rotation,
        )
            .chain()
            .in_set(FixedAppSet::CameraMovement),
    );
    app.add_systems(
        PostUpdate,
        follow_player
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate),
    );
}

fn follow_player(
    q_player: Query<&Transform, (With<Player>, Without<FirstPersonCamera>)>,
    mut q_camera: Query<&mut Transform, With<FirstPersonCamera>>,
) {
    let player_transform = single!(q_player);
    let mut camera_transform = single_mut!(q_camera);
    camera_transform.translation = player_transform.translation;
}

fn rotate_camera(mut q_camera: Query<(&mut Transform, &ActionState<CameraAction>)>) {
    let (mut transform, action) = single_mut!(q_camera);
    if let Some(axis) = action.axis_pair(&CameraAction::RotateCamera) {
        // Some machines are experiencing a continuous
        // mouse input of exactly 1.5 specifically on itch.io,
        // but not on local Wasm or native builds ¯\_ (ツ)_/¯
        const EVIL_DRIFT_VALUE: f32 = 1.5;
        const EPSILON: f32 = 0.01;
        let x = axis.x();
        let x = if (x - EVIL_DRIFT_VALUE).abs() > EPSILON {
            x
        } else {
            0.0
        };
        let y = axis.y();
        let y = if (y - EVIL_DRIFT_VALUE).abs() > EPSILON {
            y
        } else {
            0.0
        };
        let yaw = -x * 0.003;
        let pitch = -y * 0.002;
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }
}

fn clamp_rotation(
    mut q_camera: Query<&mut Transform, With<FirstPersonCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<FirstPersonCamera>)>,
) {
    let mut transform = single_mut!(q_camera);
    let player_transform = single!(q_player);
    let (_yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);

    let max_pitch = 60.0_f32.to_radians();
    let min_pitch = -40.0_f32.to_radians();
    let clamped_pitch = pitch.clamp(min_pitch, max_pitch);

    let (player_yaw, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
    // using this instead of `yaw - player_yaw` because it is guaranteed to be within `[-π, π]`
    let relative_yaw = -player_transform
        .forward()
        .xz()
        .angle_between(transform.forward().xz());
    let max_yaw = std::f32::consts::FRAC_PI_2;
    // clamp so that the yaw relative to the player is within 90 degrees
    let clamped_relative_yaw = relative_yaw.clamp(-max_yaw, max_yaw);
    let clamped_yaw = player_yaw + clamped_relative_yaw;

    let clamped_roll = 0.0;

    transform.rotation = Quat::from_euler(EulerRot::YXZ, clamped_yaw, clamped_pitch, clamped_roll);
}
