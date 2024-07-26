use std::iter;

use avian3d::prelude::*;
use bevy::{pbr::NotShadowCaster, prelude::*, render::view::RenderLayers};
use blenvy::BlueprintInstanceReady;

use crate::util::{single, single_mut};

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerViewModel>();
    app.observe(on_add_view_model);

    app.add_systems(
        PostUpdate,
        follow_player
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate),
    );
}

pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Clone, Copy, Component, Reflect, PartialEq, Eq)]
#[reflect(Debug, PartialEq, Component)]
pub struct PlayerViewModel;

fn on_add_view_model(
    trigger: Trigger<OnAdd, BlueprintInstanceReady>,
    q_view_model: Query<Entity, With<PlayerViewModel>>,
    q_parent: Query<&Parent>,
    q_children: Query<&Children>,
    q_mesh: Query<Entity, With<Handle<Mesh>>>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if !q_parent
        .get(entity)
        .is_ok_and(|e| q_view_model.contains(e.get()))
    {
        return;
    }
    let mesh_entity = iter::once(entity)
        .chain(q_children.iter_descendants(entity))
        .find_map(|e| q_mesh.get(e).ok());
    let Some(mesh_entity) = mesh_entity else {
        error!("Failed to find mesh for view model entity.");
        return;
    };

    commands.entity(mesh_entity).insert((
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        NotShadowCaster,
    ));
}

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
