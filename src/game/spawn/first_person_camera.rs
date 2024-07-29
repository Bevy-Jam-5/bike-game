//! Spawn the main level by triggering other observers.

use bevy::{core_pipeline::Skybox, prelude::*, render::view::RenderLayers};
use leafwing_input_manager::prelude::*;

use crate::game::{assets::ImageHandles, view_model::VIEW_MODEL_RENDER_LAYER};
use crate::screen::Screen;
use crate::third_party::leafwing_input_manager::CameraAction;

use super::{
    player::Player,
    ui_camera::{SpawnUiCamera, UiCamera},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(FirstPersonCamera, WorldModelCamera)>();
    app.observe(spawn_first_person_camera);
    app.observe(despawn_ui_camera);
    app.observe(spawn_ui_camera);
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct FirstPersonCamera;

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct WorldModelCamera;

pub fn base_fov() -> f32 {
    65.0_f32.to_radians()
}

#[derive(Event, Debug)]
pub struct SpawnFirstPersonCamera;

fn spawn_first_person_camera(
    _trigger: Trigger<SpawnFirstPersonCamera>,
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    image_handles: Res<ImageHandles>,
) {
    let transform = q_player.get_single().copied().unwrap_or_default();
    commands
        .spawn((
            Name::new("First Person Camera"),
            InputManagerBundle::with_map(CameraAction::default_input_map()),
            SpatialBundle::from_transform(transform),
            StateScoped(Screen::Playing),
            FirstPersonCamera,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("World Model Camera"),
                Camera3dBundle {
                    projection: PerspectiveProjection {
                        fov: base_fov(),
                        ..default()
                    }
                    .into(),
                    ..default()
                },
                Skybox {
                    image: image_handles.skybox.clone(),
                    brightness: 1200.0,
                },
                WorldModelCamera,
            ));
            parent.spawn((
                Name::new("View Model Camera"),
                Camera3dBundle {
                    camera: Camera {
                        // Bump the order to render on top of the world model.
                        order: 1,
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: 70.0_f32.to_radians(),
                        ..default()
                    }
                    .into(),
                    ..default()
                },
                IsDefaultUiCamera,
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}

fn despawn_ui_camera(
    _trigger: Trigger<SpawnFirstPersonCamera>,
    ui_camera: Query<Entity, With<UiCamera>>,
    mut commands: Commands,
) {
    for entity in &ui_camera {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_ui_camera(_trigger: Trigger<OnRemove, FirstPersonCamera>, mut commands: Commands) {
    commands.trigger(SpawnUiCamera);
}
