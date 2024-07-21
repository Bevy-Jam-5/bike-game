//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;
use iyes_progress::ProgressPlugin;

use super::Screen;
use crate::{game::assets::*, ui::prelude::*};
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ProgressPlugin::new(Screen::Loading).continue_to(Screen::Title));
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .load_collection::<LevelHandles>()
            .load_collection::<BlueprintHandles>()
            .load_collection::<AudioSfxHandles>()
            .load_collection::<AudioSoundtrackHandles>(),
    );
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
}

fn enter_loading(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}
