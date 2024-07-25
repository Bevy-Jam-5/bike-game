use bevy::prelude::*;
use delivery_zone::DeliveryZoneEntered;
use quest_marker::QuestPlace;

pub mod delivery_zone;
pub mod quest_marker;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_delivery_zone_entered);
    app.add_plugins((quest_marker::plugin, delivery_zone::plugin));
}

fn on_delivery_zone_entered(
    trigger: Trigger<DeliveryZoneEntered>,
    q_parent: Query<&Parent>,
    q_place: Query<(Entity, &QuestPlace)>,
) {
    let entity = trigger.entity();

    let Some((place_entity, place)) = q_parent
        .iter_ancestors(entity)
        .find_map(|e| q_place.get(e).ok())
    else {
        error!("Failed to get place of delivery zone entity.");
        return;
    };

    info!("Player entered delivery zone at place {:?}", place);
}
