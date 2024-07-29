use avian3d::prelude::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};

use super::CollisionLayerPreset;
pub(super) fn plugin(app: &mut App) {
    app.register_type::<DisableSensor>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
#[reflect(Debug, PartialEq, Component)]
pub struct DisableSensor;

impl Component for DisableSensor {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_insert(|mut world, entity, _component_id| {
            let mut collision_layers = world.get_mut::<CollisionLayers>(entity).unwrap();
            *collision_layers = CollisionLayers::NONE;
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            let mut collision_layers = world.get_mut::<CollisionLayers>(entity).unwrap();
            *collision_layers = CollisionLayerPreset::Sensor.into();
        });
    }
}

pub trait DisableSensorCommandsExt {
    fn disable_collider(&mut self, entity: Entity);

    fn activate_collider(&mut self, entity: Entity);
}

impl<'w, 's> DisableSensorCommandsExt for Commands<'w, 's> {
    fn disable_collider(&mut self, entity: Entity) {
        self.add(move |world: &mut World| {
            debug_assert!(
                world.get::<Collider>(entity).is_some(),
                "Cannot disable sensor of entity without collider component."
            );
            debug_assert!(
                world.get::<Sensor>(entity).is_some(),
                "Cannot disable sensor of entity without sensor component."
            );
            world.entity_mut(entity).insert(DisableSensor::default());
        });
    }

    fn activate_collider(&mut self, entity: Entity) {
        self.add(move |world: &mut World| {
            debug_assert!(
                world.get::<Collider>(entity).is_some(),
                "Cannot activate collider of entity without DisableSensor component."
            );
            world.entity_mut(entity).remove::<DisableSensor>();
        });
    }
}
