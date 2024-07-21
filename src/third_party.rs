use avian3d::prelude::*;
use bevy::prelude::*;
use blenvy::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((PhysicsPlugins::default(), BlenvyPlugin::default()));
}
