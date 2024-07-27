use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(SfxKey, SoundtrackKey)>();
}

#[derive(AssetCollection, Resource)]
pub struct LevelHandles {
    #[asset(path = "levels/World.glb")]
    _world: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct BlueprintHandles {
    #[asset(path = "blueprints/Air Conditioner.glb")]
    _air_conditioner: Handle<Gltf>,
    #[asset(path = "blueprints/Bike.glb")]
    _bike: Handle<Gltf>,
    #[asset(path = "blueprints/Box.glb")]
    _box: Handle<Gltf>,
    #[asset(path = "blueprints/Building Beige Corner Pizza.glb")]
    _building_beige_corner_pizza: Handle<Gltf>,
    #[asset(path = "blueprints/Building Beige.glb")]
    _building_beige: Handle<Gltf>,
    #[asset(path = "blueprints/Building Black.glb")]
    _building_black: Handle<Gltf>,
    #[asset(path = "blueprints/Ramp.glb")]
    _ramp: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct MaterialHandles {
    #[asset(path = "materials/Bike.glb")]
    _bike: Handle<Gltf>,
    #[asset(path = "materials/Black.glb")]
    _black: Handle<Gltf>,
    #[asset(path = "materials/Blue.glb")]
    _blue: Handle<Gltf>,
    #[asset(path = "materials/brick_shade1.glb")]
    _brick_shade1: Handle<Gltf>,
    #[asset(path = "materials/brick_shade2.glb")]
    _brick_shade2: Handle<Gltf>,
    #[asset(path = "materials/brick_shade3.glb")]
    _brick_shade3: Handle<Gltf>,
    #[asset(path = "materials/brick_shadeGreen.glb")]
    _brick_shade_green: Handle<Gltf>,
    #[asset(path = "materials/brick_shadeGreen2.glb")]
    _brick_shade_green2: Handle<Gltf>,
    #[asset(path = "materials/citybits_texture.glb")]
    _citybits_texture: Handle<Gltf>,
    #[asset(path = "materials/concrete.glb")]
    _concrete: Handle<Gltf>,
    #[asset(path = "materials/door_white.glb")]
    _door_white: Handle<Gltf>,
    #[asset(path = "materials/Dots Stroke.glb")]
    _dots_stroke: Handle<Gltf>,
    #[asset(path = "materials/Grey.glb")]
    _grey: Handle<Gltf>,
    #[asset(path = "materials/Headlights.glb")]
    _headlights: Handle<Gltf>,
    #[asset(path = "materials/metal_shade1.glb")]
    _metal_shade1: Handle<Gltf>,
    #[asset(path = "materials/OrangeWallMaterial.glb")]
    _orange_wall_material: Handle<Gltf>,
    #[asset(path = "materials/RampMaterial.glb")]
    _ramp_material: Handle<Gltf>,
    #[asset(path = "materials/red.glb")]
    _red: Handle<Gltf>,
    #[asset(path = "materials/sandstone.glb")]
    _sandstone: Handle<Gltf>,
    #[asset(path = "materials/TailLights.glb")]
    _tail_lights: Handle<Gltf>,
    #[asset(path = "materials/white.glb")]
    _white: Handle<Gltf>,
    #[asset(path = "materials/window_frame.glb")]
    _window_frame: Handle<Gltf>,
    #[asset(path = "materials/window_glass.glb")]
    _window_glass: Handle<Gltf>,
    #[asset(path = "materials/Windows.glb")]
    _windows: Handle<Gltf>,
    #[asset(path = "materials/wood.glb")]
    _wood: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioSfxHandles {
    #[asset(path = "audio/sfx/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/button_press.ogg")]
    pub button_press: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step1.ogg")]
    pub step1: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step2.ogg")]
    pub step2: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step3.ogg")]
    pub step3: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step4.ogg")]
    pub step4: Handle<AudioSource>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

#[derive(AssetCollection, Resource)]
pub struct AudioSoundtrackHandles {
    #[asset(path = "audio/soundtracks/Monkeys Spinning Monkeys.ogg")]
    pub credits: Handle<AudioSource>,
    #[asset(path = "audio/soundtracks/Fluffing A Duck.ogg")]
    pub gameplay: Handle<AudioSource>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}
