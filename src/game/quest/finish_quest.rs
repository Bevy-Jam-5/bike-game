use bevy::prelude::*;

use crate::game::quest::quest_place::QuestPlace;

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
) {
    let Some(active_quest) = active_quest.as_mut() else {
        error!("Cannot finish quest without active quest.");
        return;
    };

    let quest_giver = active_quest.quest_giver().unwrap();
    commands.entity(quest_giver.entity).remove::<QuestPlace>();

    let last_stage = active_quest.history.last().unwrap();
    info!("Finished quest at {:?}.", last_stage.place);

    commands.remove_resource::<ActiveQuest>();
}
