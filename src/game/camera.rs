use crate::util::{single, single_mut};
use crate::{third_party::leafwing_input_manager::CameraAction, FixedAppSet};
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::spawn::{first_person_camera::FirstPersonCamera, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (rotate_camera, clamp_rotation)
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
        let yaw = -axis.x() * 0.003;
        let pitch = -axis.y() * 0.002;
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }
}

fn clamp_rotation(mut q_camera: Query<&mut Transform, With<FirstPersonCamera>>) {
    let mut transform = single_mut!(q_camera);
    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let max_pitch = 60.0_f32.to_radians();
    let min_pitch = -40.0_f32.to_radians();
    if pitch > max_pitch {
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, max_pitch, roll);
    } else if pitch < min_pitch {
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, min_pitch, roll);
    }
}
