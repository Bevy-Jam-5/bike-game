use bevy::prelude::*;

use crate::game::{
    assets::SfxKey,
    audio::sfx::PlaySfx,
    quest::{commands_ext::QuestCommandsExt as _, finish_quest::FinishQuest},
};

use super::quest_place::QuestPlace;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_advance_quest);
    app.observe(on_advance_pizza_npc);
    app.observe(on_advance_mail_npc);
    app.observe(on_advance_pizzeria);
    app.observe(on_advance_post_office);
    app.register_type::<ActiveQuest>();
}

/// Only exists when an active quest is in progress.
#[derive(Debug, Resource, Clone, Eq, PartialEq, Reflect)]
#[reflect(Debug, Resource, PartialEq)]
pub struct ActiveQuest {
    pub history: Vec<FinishedStage>,
}

impl ActiveQuest {
    pub fn quest_giver(&self) -> Option<FinishedStage> {
        self.history.first().copied()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
pub struct FinishedStage {
    pub entity: Entity,
    pub place: QuestPlace,
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Event)]
#[reflect(Debug, PartialEq)]
pub struct AdvanceQuest;

fn on_advance_quest(
    trigger: Trigger<AdvanceQuest>,
    mut commands: Commands,
    q_place: Query<&QuestPlace>,
) {
    let entity = trigger.entity();
    let place = q_place.get(entity).unwrap();

    match place {
        QuestPlace::PizzaNpc => commands.trigger_targets(AdvancePizzaNpc, entity),
        QuestPlace::MailNpc => commands.trigger_targets(AdvanceMailNpc, entity),
        QuestPlace::Pizzeria => commands.trigger_targets(AdvancePizzeria, entity),
        QuestPlace::PostOffice => commands.trigger_targets(AdvancePostOffice, entity),
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Event)]
#[reflect(Debug, PartialEq)]
struct AdvancePizzaNpc;

fn on_advance_pizza_npc(
    trigger: Trigger<AdvancePizzaNpc>,
    mut commands: Commands,
    mut active_quest: Option<ResMut<ActiveQuest>>,
) {
    let entity = trigger.entity();
    if let Some(active_quest) = active_quest.as_mut() {
        // Assert this is the same NPC that gave the quest in the first place.
        debug_assert!(
            matches!(
                active_quest.quest_giver(),
                Some(FinishedStage {
                    place: QuestPlace::PizzaNpc,
                    ..
                })
            ),
            "Unexpected advance pizza NPC event."
        );
        active_quest.history.push(FinishedStage {
            entity,
            place: QuestPlace::PizzaNpc,
        });
        // Quest done
        commands.trigger(FinishQuest);
        commands.activate_all_npcs();
    } else {
        info!("Starting pizza quest. Go to the Pizzeria.");
        commands.insert_resource(ActiveQuest {
            history: vec![FinishedStage {
                entity,
                place: QuestPlace::PizzaNpc,
            }],
        });
        commands.trigger(PlaySfx::Key(SfxKey::Quest));
        commands.activate_pizzeria();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Event)]
#[reflect(Debug, PartialEq)]
struct AdvanceMailNpc;

fn on_advance_mail_npc(
    trigger: Trigger<AdvanceMailNpc>,
    mut commands: Commands,
    active_quest: Option<Res<ActiveQuest>>,
) {
    info!("Starting mail quest. Go to the Post Office.");
    let entity = trigger.entity();
    // We never return to the mail NPC, so we should never have an active quest.
    debug_assert!(active_quest.is_none(), "Unexpected advance mail NPC event.");
    commands.insert_resource(ActiveQuest {
        history: vec![FinishedStage {
            entity,
            place: QuestPlace::MailNpc,
        }],
    });
    commands.trigger(PlaySfx::Key(SfxKey::Quest));
    commands.activate_post_office();
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Event)]
#[reflect(Debug, PartialEq)]
struct AdvancePizzeria;

fn on_advance_pizzeria(
    trigger: Trigger<AdvancePizzeria>,
    mut commands: Commands,
    mut active_quest: Option<ResMut<ActiveQuest>>,
) {
    info!("Fetching pizza. Return to the Pizza NPC.");
    let entity = trigger.entity();
    debug_assert!(active_quest.is_some(), "Unexpected advance pizzeria event.");
    let Some(active_quest) = active_quest.as_mut() else {
        error!("Unexpected advance pizzeria event.");
        return;
    };
    active_quest.history.push(FinishedStage {
        entity,
        place: QuestPlace::Pizzeria,
    });
    let quest_giver = active_quest.quest_giver().unwrap();
    debug_assert_eq!(quest_giver.place, QuestPlace::PizzaNpc);
    commands.trigger(PlaySfx::Key(SfxKey::Quest));
    commands.activate_entity(quest_giver.entity);
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect, Event)]
#[reflect(Debug, PartialEq)]
struct AdvancePostOffice;

fn on_advance_post_office(
    trigger: Trigger<AdvancePostOffice>,
    mut commands: Commands,
    mut active_quest: Option<ResMut<ActiveQuest>>,
) {
    let entity = trigger.entity();
    debug_assert!(
        active_quest.is_some(),
        "Unexpected advance post office event."
    );
    let Some(active_quest) = active_quest.as_mut() else {
        error!("Unexpected advance post office event.");
        return;
    };
    active_quest.history.push(FinishedStage {
        entity,
        place: QuestPlace::PostOffice,
    });
    commands.trigger(FinishQuest);
    commands.activate_all_npcs();
}
