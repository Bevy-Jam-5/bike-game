use avian3d::prelude::*;
use bevy::prelude::*;

use crate::util::{single, single_mut};

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerViewModel>();

    app.add_systems(
        PostUpdate,
        follow_player
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate),
    );
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, PartialEq, Component)]
pub struct PlayerViewModel;

fn follow_player(
    time: Res<Time>,
    q_player: Query<&Transform, (With<Player>, Without<PlayerViewModel>)>,
    mut q_view_model: Query<&mut Transform, With<PlayerViewModel>>,
) {
    let dt = time.delta_seconds();
    let player_transform = single!(q_player);
    let mut view_model_transform = single_mut!(q_view_model);
    view_model_transform.translation = player_transform.translation;
    // Decay rate of ln(10) => after 1 second, remaining distance is 1/10th
    let decay_rate = f32::ln(10.0);
    view_model_transform.rotation = view_model_transform
        .rotation
        .slerp(player_transform.rotation, 1.0 - f32::exp(-decay_rate * dt));
}
