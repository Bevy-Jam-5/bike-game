use bevy::prelude::*;
use blenvy::{BlueprintInstanceReady, GameWorldTag};

use crate::{
    game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack},
    screen::PlayState,
    third_party::avian::DisableColliderCommandsExt as _,
};

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

    // Starting music now instead of at screen transition,
    // because otherwise Wasm will experience a delay
    // that is then compensated by running the music at a faster speed.
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}
