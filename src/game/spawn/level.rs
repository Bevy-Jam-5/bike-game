//! Spawn the main level by triggering other observers.

use bevy::{color::palettes::tailwind, prelude::*};
use blenvy::*;

use crate::{
    screen::{PlayState, Screen},
    util::single,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.observe(on_level_loaded);
    app.add_systems(Update, hack_loading.run_if(in_state(PlayState::Spawning)));
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

fn on_level_loaded(
    trigger: Trigger<OnAdd, BlueprintInstanceReady>,
    q_world: Query<&GameWorldTag>,
    mut next_state: ResMut<NextState<PlayState>>,
) {
    let entity = trigger.entity();
    if !q_world.contains(entity) {
        return;
    }

    next_state.set(PlayState::Active);
}

/// Needed because `BlueprintReadyForFinalizing` is not inserted on `World` in about 25% of runs on Wasm
/// due to a bug that is probably coming from Blenvy
fn hack_loading(
    mut commands: Commands,
    q_world: Query<
        Entity,
        (
            With<GameWorldTag>,
            With<BlueprintSpawning>,
            Without<BlueprintReadyForFinalizing>,
            Without<BlueprintInstanceReady>,
        ),
    >,
    q_blueprints: Query<Has<BlueprintInstanceReady>, (With<BlueprintInfo>, Without<GameWorldTag>)>,
) {
    let world = single!(q_world);
    if !q_blueprints.is_empty() && q_blueprints.iter().all(|ready| ready) {
        commands.entity(world).insert(BlueprintReadyForFinalizing);
    }
}
