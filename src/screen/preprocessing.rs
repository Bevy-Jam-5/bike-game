//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::{
    asset::LoadState,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

use super::Screen;
use crate::{game::assets::*, ui::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Preprocessing), enter_preprocessing);
    app.add_systems(
        Update,
        configure_skybox_texture.run_if(in_state(Screen::Preprocessing)),
    );
}

fn enter_preprocessing(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Preprocessing))
        .with_children(|children| {
            children.label("Preprocessing Assets...", fonts.rubik_regular.clone_weak());
        });
}

fn configure_skybox_texture(
    image_handles: Res<ImageHandles>,
    mut images: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<Screen>>,
    asset_server: Res<AssetServer>,
) {
    let skybox = &image_handles.skybox;
    if !matches!(asset_server.get_load_state(skybox), Some(LoadState::Loaded)) {
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
    next_state.set(Screen::Title);
}
