use std::iter;

use bevy::prelude::*;

use crate::game::{particle_emitter::ParticleEmitter, quest::quest_place::QuestPlace};

use super::advance_quest::ActiveQuest;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_finish_quest);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Event)]
#[reflect(Debug, PartialEq)]
pub struct FinishQuest;

fn on_finish_quest(
    _trigger: Trigger<FinishQuest>,
    mut commands: Commands,
    mut active_quest: Option<ResMut<ActiveQuest>>,
    q_children: Query<&Children>,
    q_particle_emitter: Query<(), With<ParticleEmitter<Extrusion<Annulus>>>>,
) {
    let Some(active_quest) = active_quest.as_mut() else {
        error!("Cannot finish quest without active quest.");
        return;
    };

    let quest_giver = active_quest.quest_giver().unwrap();
    commands.entity(quest_giver.entity).remove::<QuestPlace>();
    if let Some(emitter) = iter::once(quest_giver.entity)
        .chain(q_children.iter_descendants(quest_giver.entity))
        .find(|e| q_particle_emitter.contains(*e))
    {
        // Do not use `despawn_recursive`, particles have their own lifecycle.
        // Do not use `despawn` either, as that will leave the children dangling
        // and make Bevy unhappy, as they hold `InheritedVisibility`.
        commands
            .entity(emitter)
            .remove::<ParticleEmitter<Extrusion<Annulus>>>()
            .remove::<PointLight>();
    }

    let last_stage = active_quest.history.last().unwrap();
    info!("Finished quest at {:?}.", last_stage.place);

    commands.remove_resource::<ActiveQuest>();
}
