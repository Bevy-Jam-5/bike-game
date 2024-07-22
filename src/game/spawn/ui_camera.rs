//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<UiCamera>();
    app.observe(spawn_ui_camera);
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct UiCamera;

#[derive(Event, Debug)]
pub struct SpawnUiCamera;

fn spawn_ui_camera(_trigger: Trigger<SpawnUiCamera>, mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2dBundle::default(),
        // Render all UI to this camera.
        IsDefaultUiCamera,
        UiCamera
    ));
}
