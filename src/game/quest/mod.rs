use std::assert_matches::debug_assert_matches;

use active_quest::{ActiveQuest, PizzaQuestStage, QuestStage};
use bevy::prelude::*;
use delivery_zone::{DeliveryZone, DeliveryZoneEntered};
use quest_marker::QuestPlace;

pub mod active_quest;
pub mod delivery_zone;
pub mod quest_marker;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_delivery_zone_entered);
    app.add_plugins((
        quest_marker::plugin,
        delivery_zone::plugin,
        active_quest::plugin,
    ));
}

fn on_delivery_zone_entered(
    mut commands: Commands,
    trigger: Trigger<DeliveryZoneEntered>,
    q_parent: Query<&Parent>,
    q_place: Query<(Entity, &QuestPlace)>,
    mut active_quest: Option<ResMut<ActiveQuest>>,
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
    match place {
        QuestPlace::PizzaNpc => {
            if let Some(active_quest) = active_quest.as_mut() {
                debug_assert!(
                    matches!(
                        active_quest.stage,
                        QuestStage::Pizza(PizzaQuestStage::GetPizzaAtPizzeria)),
                    "Unexpected quest stage: {:?}", active_quest.stage
                );
                // Quest done
                commands.entity(place_entity).remove::<QuestPlace>();
                |(_, place)| place.is_npc()
            } else {
                commands.insert_resource(ActiveQuest {
                    giver: place_entity,
                    stage: PizzaQuestStage::GetPizzaAtPizzeria.into(),
                });
                |(_, place)| place.is_pizzeria()
            }
        }
        QuestPlace::MailNpc => {
            debug_assert!(active_quest.is_none(), "Unexpected active quest: {:?}", active_quest);
            commands.insert_resource(ActiveQuest {
                giver: place_entity,
                stage: MailQuestStage::DeliverMailAtPostOffice.into(),
            });
            |(_, place)| place.is_post_office()
        }
        QuestPlace::Pizzeria => {
            |(e, _)| e == active_quest.giver.unwrap()
        }
        QuestPlace::PostOffice => {
            |(e, _)| e == active_quest.giver.unwrap()
        }
    }
}


trait QuestCommandsExt {
    fn activate_all_npcs(&mut self);
    fn activate_entity(&mut self, entity: Entity);
    fn activate_pizzeria(&mut self);
    fn activate_post_office(&mut self);
}

impl<'w, 's> QuestCommandsExt for Commands<'w, 's> {
    fn activate_all_npcs(&mut self) {
        self.add(|world: &mut World| {
            todo!();
        });
    }

    fn activate_entity(&mut self, entity: Entity) {
        self.add(|world: &mut World| {
            todo!();
        });
    }

    fn activate_pizzeria(&mut self) {
        self.add(|world: &mut World| {
            todo!();
        });
    }

    fn activate_post_office(&mut self) {
        self.add(|world: &mut World| {
            todo!();
        });
    }

}
