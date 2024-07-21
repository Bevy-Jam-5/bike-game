use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::third_party::leafwing_input_manager::PlayerAction;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.observe(on_player_spawn);
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct Player;

fn on_player_spawn(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(InputManagerBundle::with_map(
            PlayerAction::default_input_map(),
        ));
}
