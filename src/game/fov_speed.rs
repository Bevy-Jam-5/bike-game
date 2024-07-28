use avian3d::prelude::*;
use bevy::prelude::*;

use crate::util::{single, single_mut};

use super::spawn::{
    first_person_camera::{base_fov, WorldModelCamera},
    player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, adjust_fov_to_speed);
}

fn adjust_fov_to_speed(
    time: Res<Time>,
    q_player: Query<&LinearVelocity, With<Player>>,
    mut q_camera: Query<&mut Projection, With<WorldModelCamera>>,
) {
    let velocity = single!(q_player);
    let mut projection = single_mut!(q_camera);
    let dt = time.delta_seconds();
    let Projection::Perspective(ref mut perspective) = *projection else {
        error!("World model camera has no perspective projection?!");
        return;
    };

    // Intentionally not necessarily the same as max_pedal_speed
    let speed_for_max_fov = 16.0;
    let max_additional_fov = 45.0_f32.to_radians();

    let speed = velocity.0.length();
    let factor = speed / speed_for_max_fov;
    let factor_scaled = factor * factor;
    let factor_unjittered = if factor_scaled < 0.1 {
        0.0
    } else if factor_scaled > 0.9 {
        1.0
    } else {
        factor_scaled
    };

    let additional_fov = max_additional_fov * factor_unjittered;
    let origin = perspective.fov;
    let target = base_fov() + additional_fov;
    let decay_rate = f32::ln(10.0);
    perspective.fov = origin.lerp(target, 1.0 - f32::exp(-decay_rate * dt));
}
