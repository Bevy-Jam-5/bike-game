use bevy::prelude::*;

pub mod advance_quest;
pub mod commands_ext;
pub mod delivery_zone;
pub mod finish_quest;
pub mod init_quests;
pub mod quest_place;
pub mod lights;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        quest_place::plugin,
        delivery_zone::plugin,
        advance_quest::plugin,
        commands_ext::plugin,
        finish_quest::plugin,
        init_quests::plugin,
        lights::plugin,
    ));
}
