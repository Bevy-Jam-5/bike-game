use std::iter;

use crate::third_party::avian::DisableCollider;
use crate::AppSet;
use avian3d::prelude::*;
use bevy::prelude::*;

use super::{advance_quest::AdvanceQuest, quest_place::QuestPlace};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_create_delivery_zone);
    app.register_type::<(DeliveryZone, DeliveryZoneLink)>();
    app.add_systems(
        Update,
        on_delivery_player_collision.in_set(AppSet::ReadCollisions),
    );
}

/// Marker for a delivery zone for quest advancement.
/// Created automatically by inserting a [`QuestPlace`] on an entity.
#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub struct DeliveryZone;

/// Link on [`QuestPlace`] pointing to the delivery zone entity holding the actual collider.
#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub struct DeliveryZoneLink(pub Entity);

/// Add [`DeliveryZoneLink`] to the [`QuestPlace`] entity.
/// Not a hook because `iter_ancestors` does not work there :(
fn on_create_delivery_zone(
    trigger: Trigger<OnAdd, DeliveryZone>,
    mut commands: Commands,
    q_parent: Query<&Parent>,
    q_place: Query<Entity, With<QuestPlace>>,
) {
    let entity = trigger.entity();
    let place_entity = iter::once(entity)
        .chain(q_parent.iter_ancestors(entity))
        .find(|&e| q_place.contains(e));
    let Some(place_entity) = place_entity else {
        error!("Failed to get place of delivery zone entity.");
        return;
    };
    commands
        .entity(place_entity)
        .insert(DeliveryZoneLink(entity));
}

fn on_delivery_player_collision(
    mut commands: Commands,
    q_delivery_zone: Query<
        (Entity, &CollidingEntities),
        (With<DeliveryZone>, Without<DisableCollider>),
    >,
    q_parent: Query<&Parent>,
    q_place: Query<Entity, With<QuestPlace>>,
) {
    for (entity, collisions) in q_delivery_zone.iter() {
        for _ in collisions.iter() {
            // Only the player can collide with delivery zones,
            // so no need to check the actual colliding entity.
            let Some(place_entity) = q_parent
                .iter_ancestors(entity)
                .find_map(|e| q_place.get(e).ok())
            else {
                error!("Failed to get place of delivery zone entity.");
                return;
            };
            commands.trigger_targets(AdvanceQuest, place_entity);
        }
    }
}
