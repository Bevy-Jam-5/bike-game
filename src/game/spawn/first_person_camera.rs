//! Spawn the main level by triggering other observers.

use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        view::RenderLayers,
    },
};
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
    app.add_systems(
        Update,
        configure_skybox_texture
            .run_if(resource_exists::<ImageHandles>)
            .run_if(on_event::<AssetEvent<Image>>()),
    );
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
                IsDefaultUiCamera,
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

fn configure_skybox_texture(
    mut asset_events: EventReader<AssetEvent<Image>>,
    image_handles: Res<ImageHandles>,
    mut images: ResMut<Assets<Image>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    for event in asset_events.read() {
        let skybox = &image_handles.skybox;
        if !event.is_loaded_with_dependencies(skybox) {
            return;
        }
        let image = images.get_mut(skybox).unwrap();
        // Note: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        // We could use ktx2, but generating it with gltf-ibl-sampler-egui made the sky too oversaturated.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }
        *done = true;
    }
}
