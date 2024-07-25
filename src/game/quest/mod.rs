use bevy::prelude::*;

pub mod active_quest;
pub mod commands_ext;
pub mod delivery_zone;
pub mod quest_marker;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        quest_marker::plugin,
        delivery_zone::plugin,
        active_quest::plugin,
        commands_ext::plugin,
    ));
}
