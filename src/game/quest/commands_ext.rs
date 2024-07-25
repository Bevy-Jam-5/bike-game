use bevy::prelude::*;

use crate::third_party::avian::DisableColliderCommandsExt;

use super::quest_marker::QuestPlace;

pub(super) fn plugin(_app: &mut App) {}

pub trait QuestCommandsExt {
    fn activate_all_npcs(&mut self);
    fn activate_entity(&mut self, entity: Entity);
    fn activate_pizzeria(&mut self);
    fn activate_post_office(&mut self);
    fn activate_if(&mut self, predicate: impl Fn((Entity, QuestPlace)) -> bool + Send + 'static);
}

impl<'w, 's> QuestCommandsExt for Commands<'w, 's> {
    fn activate_all_npcs(&mut self) {
        self.activate_if(|(_e, place)| place.is_npc());
    }

    fn activate_entity(&mut self, entity: Entity) {
        self.activate_if(move |(e, _place)| e == entity);
    }

    fn activate_pizzeria(&mut self) {
        self.activate_if(|(_e, place)| place.is_pizzeria());
    }

    fn activate_post_office(&mut self) {
        self.activate_if(|(_e, place)| place.is_post_office());
    }

    fn activate_if(&mut self, predicate: impl Fn((Entity, QuestPlace)) -> bool + Send + 'static) {
        self.add(move |world: &mut World| {
            let mut q_place = world.query::<(Entity, &QuestPlace)>();
            let places = q_place
                .iter(world)
                .map(|(e, &place)| (e, place))
                .collect::<Vec<_>>();
            for (entity, place) in places {
                if predicate((entity, place)) {
                    world.commands().activate_collider(entity);
                } else {
                    world.commands().disable_collider(entity);
                }
            }
        });
    }
}
