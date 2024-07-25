use bevy::prelude::*;

pub mod delivery_zone;
pub mod npc;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((npc::plugin, delivery_zone::plugin));
}
