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
    #[asset(path = "blueprints/ColliderCube.glb")]
    _collider_cube: Handle<Gltf>,
    #[asset(path = "blueprints/Crate.glb")]
    _crate: Handle<Gltf>,
    #[asset(path = "blueprints/Ramp.glb")]
    _ramp: Handle<Gltf>,
    #[asset(path = "blueprints/Wall.glb")]
    _wall: Handle<Gltf>,
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
