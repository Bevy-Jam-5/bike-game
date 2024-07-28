use avian3d::prelude::*;
use bevy::{prelude::*, utils::HashSet};

use crate::util::single;

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, yeet_props);
    app.register_type::<YeetCollider>();
}

#[derive(Debug, Clone, Copy, Reflect, Component)]
#[reflect(Debug, Component)]
pub struct YeetCollider;

fn yeet_props(
    q_player: Query<&LinearVelocity, With<Player>>,
    q_yeeter: Query<&CollidingEntities, With<YeetCollider>>,
    q_collider: Query<&ColliderParent>,
    mut q_rigid_body: Query<(&RigidBody, &mut ExternalImpulse)>,
    mut last_collisions: Local<HashSet<Entity>>,
) {
    let colliding_entities = single!(q_yeeter);
    let velocity = single!(q_player);
    let new_collisions: Vec<_> = colliding_entities
        .0
        .difference(&last_collisions)
        .copied()
        .collect();
    *last_collisions = colliding_entities.0.clone();
    for entity in new_collisions {
        let Ok(collider_parent) = q_collider.get(entity) else {
            error!("Player collided with a non-collider?!");
            continue;
        };
        let Ok((rigid_body, mut external_impulse)) = q_rigid_body.get_mut(collider_parent.get())
        else {
            error!("Collider parent has no rigid body?!");
            continue;
        };
        if !matches!(rigid_body, RigidBody::Dynamic) {
            continue;
        }
        // Unit is Nsm⁻¹s⁻¹
        const IMPULSE_FACTOR: f32 = 1.5;
        let impulse = velocity.0 * IMPULSE_FACTOR;
        external_impulse.apply_impulse(impulse);
    }
}
