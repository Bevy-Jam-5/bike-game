//! Spawn the main level by triggering other observers.

use bevy::{color::palettes::tailwind, prelude::*};
use blenvy::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.spawn((
        Name::new("Level"),
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
        StateScoped(Screen::Playing),
    ));
    commands.insert_resource(AmbientLight {
        color: tailwind::ORANGE_100.into(),
        brightness: 70.0,
    });
}
