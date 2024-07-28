use bevy::prelude::*;
use bevy_tnua::controller::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use blenvy::*;

pub mod avian;
pub mod leafwing_input_manager;
pub mod pipelines_ready;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaControllerPlugin::default(),
        TnuaAvian3dPlugin::default(),
        BlenvyPlugin {
            export_registry: cfg!(feature = "dev_native"),
            ..default()
        },
        leafwing_input_manager::plugin,
        avian::plugin,
        pipelines_ready::plugin,
    ));
}
