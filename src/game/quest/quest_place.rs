use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use blenvy::*;
use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<QuestPlace>();
}

/// The different places where a quest can take place.
/// Automatically spawns a [`DeliveryZone`](super::delivery_zone::DeliveryZone) when added to an entity,
/// which is easily accessed through the [`DeliveryZoneLink`](super::delivery_zone::DeliveryZoneLink) component.
#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub enum QuestPlace {
    PizzaNpc,
    MailNpc,
    Pizzeria,
    PostOffice,
}

impl QuestPlace {
    pub fn is_npc(self) -> bool {
        matches!(self, Self::PizzaNpc | Self::MailNpc)
    }

    pub fn is_pizzeria(self) -> bool {
        matches!(self, Self::Pizzeria)
    }

    pub fn is_post_office(self) -> bool {
        matches!(self, Self::PostOffice)
    }

    pub fn description(self) -> String {
        let sentences = match self {
            QuestPlace::PizzaNpc => vec![
                "\"I'm hungry, bring me a pizza!\"",
                "\"I need a pizza, can you get me one?\"",
                "\"I'm craving pizza!\"",
                "\"It is of utmost importance that I receive a pizza. Shall you fetch me one?\"",
                "\"Need pizza. Now.\"",
                "\"Ey yo, pizza time! Bring me a fresh one!\"",
                "\"I'm starving, can you get me a pizza?\"",
                "\"So hungry... pizza... please...\"",
            ],
            QuestPlace::Pizzeria => {
                vec!["Deliver the pizza back to the customer before it gets cold!"]
            }
            QuestPlace::MailNpc => {
                vec![
                    "\"I need to send this letter, can you bring it to the mailbox?",
                    "\"I have a letter that needs to be sent, can you help me out?\"",
                    "\"No time to explain, just take this letter to the mailbox!\"",
                    "\"I need this letter sent, it's urgent!\"",
                    "\"Post haste! Take this letter to the mailbox!\"",
                    "\"I need this letter sent, can you do it for me?\"",
                    "\"Please please please take this letter to the mailbox!\"",
                    "\"Help! I need this letter sent RIGHT NOW!\"",
                ]
            }
            QuestPlace::PostOffice => {
                // We never return to the mail NPC, so we should never have an active quest.
                error!("Unexpected advance mail event");
                Vec::new()
            }
        };

        // pick one at random
        sentences
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

impl Component for QuestPlace {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let mut commands = world.commands();
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    DeliveryZoneBlueprint,
                    BlueprintInfo::from_path("blueprints/DeliveryZone.glb"),
                    SpawnBlueprint,
                    TransformBundle::default(),
                ));
            });
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            let children = world.get::<Children>(entity).unwrap();
            let to_remove = children
                .iter()
                .filter(|child| world.get::<DeliveryZoneBlueprint>(**child).is_some())
                .copied()
                .collect::<Vec<_>>();
            for entity in to_remove {
                world.commands().entity(entity).despawn_recursive();
            }
        });
    }
}

#[derive(Debug, Component)]
struct DeliveryZoneBlueprint;
