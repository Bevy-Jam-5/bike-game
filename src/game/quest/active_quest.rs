use bevy::prelude::*;

use crate::game::quest::commands_ext::QuestCommandsExt as _;

use super::quest_marker::QuestPlace;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_advance_quest);
    app.observe(on_advance_pizza_npc);
    app.observe(on_advance_mail_npc);
    app.observe(on_advance_pizzeria);
    app.observe(on_advance_post_office);
    app.register_type::<ActiveQuest>();
}

#[derive(Debug, Resource, Clone, Eq, PartialEq, Reflect)]
#[reflect(Debug, Resource, PartialEq)]
pub struct ActiveQuest {
    pub history: Vec<FinishedStage>,
}

#[derive(Debug, Clone, Eq, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
pub struct FinishedStage {
    entity: Entity,
    place: QuestPlace,
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
                active_quest.history.first(),
                Some(&FinishedStage {
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
        commands.activate_all_npcs();
    } else {
        commands.insert_resource(ActiveQuest {
            history: vec![FinishedStage {
                entity,
                place: QuestPlace::PizzaNpc,
            }],
        });
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
    let entity = trigger.entity();
    // We never return to the mail NPC, so we should never have an active quest.
    debug_assert!(active_quest.is_none(), "Unexpected advance mail NPC event.");
    commands.insert_resource(ActiveQuest {
        history: vec![FinishedStage {
            entity,
            place: QuestPlace::MailNpc,
        }],
    });
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
    let quest_giver = active_quest.history.first().unwrap();
    debug_assert_eq!(quest_giver.place, QuestPlace::PizzaNpc);
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
    // Quest done
    commands.activate_all_npcs();
}
