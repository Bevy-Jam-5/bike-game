use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use blenvy::*;

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
