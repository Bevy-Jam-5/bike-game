use crate::third_party::avian::DisableCollider;
use crate::AppSet;
use avian3d::prelude::*;
use bevy::prelude::*;

use super::{advance_quest::AdvanceQuest, quest_place::QuestPlace};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DeliveryZone>();
    app.add_systems(
        Update,
        on_delivery_player_collision.in_set(AppSet::ReadCollisions),
    );
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub struct DeliveryZone;

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
            // Only the player can collide with delivery zones.

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
