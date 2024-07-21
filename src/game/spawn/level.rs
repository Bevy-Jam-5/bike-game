//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use blenvy::*;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // here we spawn our game world/level, which is also a blueprint !
    commands.spawn((
        Name::new("Level"),
        BlueprintInfo::from_path("levels/World.glb"), // all we need is a Blueprint info...
        SpawnBlueprint, // and spawnblueprint to tell blenvy to spawn the blueprint now
        HideUntilReady, // only reveal the level once it is ready
        GameWorldTag,
    ));
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}
