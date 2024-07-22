use crate::third_party::leafwing_input_manager::CameraAction;
use crate::util::{single, single_mut};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::spawn::{first_person_camera::FirstPersonCamera, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (follow_player, rotate_camera).chain());
}

fn follow_player(
    time: Res<Time>,
    q_player: Query<&Transform, (With<Player>, Without<FirstPersonCamera>)>,
    mut q_camera: Query<&mut Transform, With<FirstPersonCamera>>,
) {
    let player_transform = single!(q_player);
    let mut camera_transform = single_mut!(q_camera);
    let dt = time.delta_seconds();

    let start = camera_transform.translation;
    let target = player_transform.translation;
    const DECAY_RATE: f32 = 2.0;

    // Source: <https://github.com/bevyengine/bevy/blob/08d3497d87f02005603116866ec6730fb05a7445/crates/bevy_math/src/common_traits.rs#L259C9-L259C85>
    camera_transform.translation = start.lerp(target, 1.0 - f32::exp(-DECAY_RATE * dt));
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
