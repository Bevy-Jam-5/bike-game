use avian3d::prelude::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CollisionLayerPreset>();
}

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Default,
    Player,
    DeliveryZone,
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, PartialEq, Component)]
pub enum CollisionLayerPreset {
    Default,
    Player,
    Sensor,
}

impl From<CollisionLayerPreset> for CollisionLayers {
    fn from(preset: CollisionLayerPreset) -> Self {
        match preset {
            CollisionLayerPreset::Default => CollisionLayers::new(
                CollisionLayer::Default,
                [CollisionLayer::Default, CollisionLayer::Player],
            ),
            CollisionLayerPreset::Player => CollisionLayers::new(
                CollisionLayer::Player,
                [CollisionLayer::Default, CollisionLayer::DeliveryZone],
            ),
            CollisionLayerPreset::Sensor => {
                CollisionLayers::new(CollisionLayer::DeliveryZone, CollisionLayer::Player)
            }
        }
    }
}

impl Component for CollisionLayerPreset {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            let preset = *world.get::<CollisionLayerPreset>(entity).unwrap();
            let mut commands = world.commands();
            commands
                .entity(entity)
                .insert(CollisionLayers::from(preset));
        });
    }
}
