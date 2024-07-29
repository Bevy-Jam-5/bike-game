use bevy::prelude::*;

use crate::{screen::PlayState, third_party::avian::DisableSensorCommandsExt as _};

use super::{delivery_zone::DeliveryZoneLink, quest_place::QuestPlace};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Active), init_quests);
}

fn init_quests(q_quest_places: Query<(&QuestPlace, &DeliveryZoneLink)>, mut commands: Commands) {
    for (quest_place, delivery_zone) in q_quest_places.iter() {
        // At the start of the game, only enable colliders for quest givers.
        if !quest_place.is_npc() {
            commands.disable_collider(delivery_zone.0);
        }
    }
}
