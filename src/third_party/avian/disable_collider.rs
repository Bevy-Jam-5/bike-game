use avian3d::prelude::*;
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
pub(super) fn plugin(app: &mut App) {
    app.register_type::<DisableCollider>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
#[reflect(Debug, PartialEq, Component)]
pub struct DisableCollider {
    pub previous_collision_layers: Option<CollisionLayers>,
}

impl Component for DisableCollider {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            {
                let previous_collision_layers = *world.get::<CollisionLayers>(entity).unwrap();
                let mut disable_collider = world.get_mut::<DisableCollider>(entity).unwrap();
                disable_collider.previous_collision_layers = Some(previous_collision_layers);
            }
            let mut collision_layers = world.get_mut::<CollisionLayers>(entity).unwrap();
            *collision_layers = CollisionLayers::NONE;
        });

        hooks.on_remove(|mut world, entity, _component_id| {
            let disable_collider = world.get::<DisableCollider>(entity).unwrap();
            let previous_collision_layers = disable_collider.previous_collision_layers.unwrap();
            let mut collision_layers = world.get_mut::<CollisionLayers>(entity).unwrap();
            *collision_layers = previous_collision_layers;
        });
    }
}

pub trait DisableColliderCommandsExt {
    fn disable_collider(&mut self, entity: Entity);
}

impl<'w, 's> DisableColliderCommandsExt for Commands<'w, 's> {
    fn disable_collider(&mut self, entity: Entity) {
        self.add(move |world: &mut World| {
            world.entity_mut(entity).insert(DisableCollider::default());
        });
    }
}
