use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<QuestNpc>();
}


#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
enum QuestNpc {
    Pizza,
    Mail
}
