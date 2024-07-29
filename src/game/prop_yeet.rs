use std::time::Duration;

use avian3d::prelude::*;
use bevy::{prelude::*, utils::HashSet};
use rand::Rng;

use crate::{
    game::{assets::SfxKey, audio::sfx::PlaySfx},
    util::single,
};

use super::{spawn::player::Player, time::RemainingTime};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, yeet_props);
    app.register_type::<YeetCollider>();
}

/// How much time is regained from an awesome yeet :D
pub const AWESOME_YEET_TIME_GAIN: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, Copy, Reflect, Component)]
#[reflect(Debug, Component)]
pub struct YeetCollider;

fn yeet_props(
    q_player: Query<&LinearVelocity, With<Player>>,
    q_yeeter: Query<&CollidingEntities, With<YeetCollider>>,
    q_collider: Query<&ColliderParent>,
    mut q_rigid_body: Query<(
        &RigidBody,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
    )>,
    mut remaining_time: ResMut<RemainingTime>,
    mut last_collisions: Local<HashSet<Entity>>,
    mut commands: Commands,
) {
    let colliding_entities = single!(q_yeeter);
    let velocity = single!(q_player);
    let new_collisions: Vec<_> = colliding_entities
        .0
        .difference(&last_collisions)
        .copied()
        .collect();
    *last_collisions = colliding_entities.0.clone();
    let mut any_awesome = false;
    for entity in new_collisions {
        let Ok(collider_parent) = q_collider.get(entity) else {
            error!("Player collided with a non-collider?!");
            continue;
        };
        let Ok((rigid_body, mut external_impulse, mut external_angular_impulse)) =
            q_rigid_body.get_mut(collider_parent.get())
        else {
            error!("Collider parent has no rigid body?!");
            continue;
        };
        if !matches!(rigid_body, RigidBody::Dynamic) {
            continue;
        }
        // Unit is kg. Not using the player's mass as that would be wayyyy too much.
        const YEETING_MASS: f32 = 1.5;
        let impulse = velocity.0 * YEETING_MASS;
        external_impulse.apply_impulse(impulse);

        let torque_dir = random_unit_vec(&mut rand::thread_rng());
        let player_speed = velocity.0.length();
        // Unit is Ns
        const ANGULAR_YEET_FACTOR: f32 = 0.3;
        let torque = torque_dir * player_speed * ANGULAR_YEET_FACTOR;
        external_angular_impulse.apply_impulse(torque);

        const AWESOME_SPEED: f32 = 12.0;
        if player_speed > AWESOME_SPEED {
            any_awesome = true;
            remaining_time.0 += AWESOME_YEET_TIME_GAIN;
        }
    }
    if any_awesome {
        commands.trigger(PlaySfx::Key(SfxKey::Yeet));
    }
}

fn random_unit_vec(rng: &mut impl Rng) -> Vec3 {
    let (x, y, z) = (
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    );
    Vec3::new(x, y, z).normalize()
}
