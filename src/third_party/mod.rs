use avian3d::prelude::*;
use bevy::prelude::*;
use blenvy::*;

pub mod leafwing_input_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        BlenvyPlugin::default(),
        leafwing_input_manager::plugin,
    ));
}
