use bevy::prelude::*;
use blenvy::{BlueprintInstanceReady, GameWorldTag};

use crate::third_party::avian::DisableColliderCommandsExt as _;

use super::{delivery_zone::DeliveryZoneLink, quest_place::QuestPlace};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_level_loaded);
}

fn on_level_loaded(
    trigger: Trigger<OnAdd, BlueprintInstanceReady>,
    q_world: Query<&GameWorldTag>,
    q_quest_places: Query<(&QuestPlace, &DeliveryZoneLink)>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if !q_world.contains(entity) {
        return;
    }

    for (quest_place, delivery_zone) in q_quest_places.iter() {
        // At the start of the game, only enable colliders for quest givers.
        if !quest_place.is_npc() {
            commands.disable_collider(delivery_zone.0);
        }
    }
}