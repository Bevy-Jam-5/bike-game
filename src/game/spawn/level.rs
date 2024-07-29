//! Spawn the main level by triggering other observers.

use bevy::{
    color::palettes::tailwind,
    prelude::*,
    utils::{HashMap, HashSet},
};
use blenvy::*;

use crate::screen::{PlayState, Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LoadTimer>();
    app.observe(spawn_level);
    app.add_systems(
        Update,
        (hack_loading, on_level_loaded)
            .chain()
            .run_if(in_state(PlayState::Spawning)),
    );
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
        color: tailwind::SKY_100.into(),
        brightness: 400.0,
    });
}

fn on_level_loaded(
    q_world: Query<&GameWorldTag, With<BlueprintInstanceReady>>,
    mut next_state: ResMut<NextState<PlayState>>,
) {
    if q_world.is_empty() {
        return;
    }
    next_state.set(PlayState::Tutorial);
}

#[derive(Debug, Resource, Deref, DerefMut)]
struct LoadTimer(Timer);

impl Default for LoadTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(10.0, TimerMode::Once))
    }
}

/// Needed because `BlueprintReadyForFinalizing` is not inserted on `World` in about 25% of runs on Wasm
/// due to a bug that is probably coming from Blenvy
fn hack_loading(
    time: Res<Time>,
    mut commands: Commands,
    q_loading: Query<
        Entity,
        (
            With<BlueprintSpawning>,
            Without<BlueprintReadyForFinalizing>,
            Without<BlueprintInstanceReady>,
        ),
    >,
    q_children: Query<&Children>,
    q_blueprints: Query<Has<BlueprintInstanceReady>, With<BlueprintInfo>>,
    mut load_timer: ResMut<LoadTimer>,
) {
    if !load_timer.finished() {
        load_timer.tick(time.delta());
        return;
    }
    if q_loading.is_empty() {
        return;
    }
    let mut processed = HashSet::new();
    let mut ready_map = HashMap::new();
    for loading in q_loading.iter() {
        if processed.contains(&loading) {
            continue;
        }
        go_through_children(
            &mut commands,
            &q_children,
            &q_blueprints,
            loading,
            &mut ready_map,
        );
        processed.insert(loading);
    }
}

fn go_through_children(
    commands: &mut Commands,
    q_children: &Query<&Children>,
    q_blueprints: &Query<Has<BlueprintInstanceReady>, With<BlueprintInfo>>,
    entity: Entity,
    ready_map: &mut HashMap<Entity, bool>,
) -> bool {
    if q_blueprints.contains(entity) && !q_children.contains(entity) {
        ready_map.insert(entity, false);
        return false;
    }
    match q_children.get(entity) {
        Ok(children) => {
            let ready = children.iter().all(|child| {
                if let Some(ready) = ready_map.get(child) {
                    *ready
                } else {
                    let ready =
                        go_through_children(commands, q_children, q_blueprints, *child, ready_map);
                    ready_map.insert(*child, ready);
                    ready
                }
            });
            if ready {
                ready_map.insert(entity, true);
                commands.entity(entity).insert(BlueprintReadyForFinalizing);
            }
            ready
        }
        Err(_) => true,
    }
}
