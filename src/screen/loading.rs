//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{game::assets::*, ui::prelude::*};
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Fonts must be loaded for title screen.
    app.add_loading_state(LoadingState::new(Screen::Splash).load_collection::<FontHandles>());

    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Preprocessing)
            .load_collection::<LevelHandles>()
            .load_collection::<BlueprintHandles>()
            .load_collection::<AudioSfxHandles>()
            .load_collection::<AudioSoundtrackHandles>()
            .load_collection::<ImageHandles>()
            .load_collection::<MaterialHandles>(),
    );
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
}

fn enter_loading(mut commands: Commands, fonts: Res<FontHandles>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...", fonts.rubik_regular.clone_weak());
        });
}
