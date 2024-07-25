use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use blenvy::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<QuestPlace>();
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub enum QuestPlace {
    PizzaNpc,
    MailNpc,
    Pizzeria,
    PostOffice,
}


impl Component for QuestPlace {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let mut commands = world.commands();
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    BlueprintInfo::from_path("blueprints/DeliveryZone.glb"),
                    SpawnBlueprint,
                    TransformBundle::default(),
                ));
            });
        });
    }
}

