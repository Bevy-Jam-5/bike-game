use crate::third_party::avian::{DisableCollider, DisableColliderCommandsExt as _};
use crate::AppSet;
use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DeliveryZone>();
    app.add_systems(
        Update,
        on_delivery_player_collision.in_set(AppSet::ReadCollisions),
    );
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
struct DeliveryZone;

#[derive(Debug, Event, Clone, Copy, PartialEq, Eq)]
struct DeliveryZoneEnteredEvent;

fn on_delivery_player_collision(
    q_delivery_zone: Query<
        (Entity, &CollidingEntities),
        (With<DeliveryZone>, Without<DisableCollider>),
    >,
    mut commands: Commands,
) {
    for (entity, collisions) in q_delivery_zone.iter() {
        for _ in collisions.iter() {
            // Only player can collide with delivery zones.
            commands.trigger_targets(DeliveryZoneEnteredEvent, entity);
            commands.disable_collider(entity);
            info!("Player entered delivery zone.");
        }
    }
}
