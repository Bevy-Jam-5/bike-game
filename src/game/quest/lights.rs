use bevy::{prelude::*, render::view::RenderLayers};

use crate::game::view_model::VIEW_MODEL_RENDER_LAYER;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_point_light_add);
    app.observe(on_spot_light_add);
    app.observe(on_directional_light_add);
}

fn on_point_light_add(trigger: Trigger<OnAdd, PointLight>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(light_render_layers());
}

fn on_spot_light_add(trigger: Trigger<OnAdd, SpotLight>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(light_render_layers());
}

fn on_directional_light_add(trigger: Trigger<OnAdd, DirectionalLight>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(light_render_layers());
}

fn light_render_layers() -> RenderLayers {
    RenderLayers::from_layers(&[0, VIEW_MODEL_RENDER_LAYER])
}
