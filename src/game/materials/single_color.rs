use bevy::{
    asset::ReflectAsset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
#[reflect(Asset, Default, Debug)]
pub struct SingleColorMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Default for SingleColorMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::WHITE,
        }
    }
}

impl From<Color> for SingleColorMaterial {
    fn from(value: Color) -> Self {
        Self {
            color: value.into(),
        }
    }
}

impl Material for SingleColorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/single_color_material.wgsl".into()
    }
}
