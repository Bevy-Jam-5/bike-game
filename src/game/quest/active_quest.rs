use bevy::prelude::*;
use blenvy::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ActiveQuest>();
}

#[derive(Debug, Resource, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Debug, Resource, PartialEq)]
pub struct ActiveQuest {
    pub giver: Entity,
    pub stage: QuestStage,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
pub enum QuestStage {
    Pizza(PizzaQuestStage),
    Mail(MailQuestStage),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
pub enum PizzaQuestStage {
    GetPizzaAtPizzeria,
    DeliverPizzaToNpc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
pub enum MailQuestStage {
    DeliverMailAtPostOffice,
}

impl From<PizzaQuestStage> for QuestStage {
    fn from(stage: PizzaQuestStage) -> Self {
        QuestStage::Pizza(stage)
    }
}

impl From<MailQuestStage> for QuestStage {
    fn from(stage: MailQuestStage) -> Self {
        QuestStage::Mail(stage)
    }
}


