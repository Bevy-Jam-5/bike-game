mod single_color;

pub use single_color::SingleColorMaterial;

use bevy::{app::App, asset::Handle, pbr::MaterialPlugin};

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<(SingleColorMaterial, Handle<SingleColorMaterial>)>();
    app.add_plugins(MaterialPlugin::<SingleColorMaterial>::default());
}
