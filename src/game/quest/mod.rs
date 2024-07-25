use bevy::prelude::*;

pub mod advance_quest;
pub mod commands_ext;
pub mod delivery_zone;
pub mod quest_place;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        quest_place::plugin,
        delivery_zone::plugin,
        advance_quest::plugin,
        commands_ext::plugin,
    ));
}
