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
    // air conditioner
    #[asset(path = "blueprints/Air Conditioner.glb")]
    _air_conditioner: Handle<Gltf>,
    // bench
    #[asset(path = "blueprints/Bench.glb")]
    _bench: Handle<Gltf>,
    // bike
    #[asset(path = "blueprints/Bike.glb")]
    _bike: Handle<Gltf>,
    // box
    #[asset(path = "blueprints/Box.glb")]
    _box: Handle<Gltf>,
    // building beige corner pizza
    #[asset(path = "blueprints/Building Beige Corner Pizza.glb")]
    _building_beige_corner_pizza: Handle<Gltf>,
    // building beige
    #[asset(path = "blueprints/Building Beige.glb")]
    _building_beige: Handle<Gltf>,
    // building black
    #[asset(path = "blueprints/Building Black.glb")]
    _building_black: Handle<Gltf>,
    // building red corner
    #[asset(path = "blueprints/Building Red Corner.glb")]
    _building_red_corner: Handle<Gltf>,
    // building red
    #[asset(path = "blueprints/Building Red.glb")]
    _building_red: Handle<Gltf>,
    // building red barren
    #[asset(path = "blueprints/Building Red Barren.glb")]
    _building_red_barren: Handle<Gltf>,
    // building big (yes, there is a typo)
    #[asset(path = "blueprints/Buliding Big.glb")]
    _building_big: Handle<Gltf>,
    // bush
    #[asset(path = "blueprints/Bush.glb")]
    _bush: Handle<Gltf>,
    // car blue
    #[asset(path = "blueprints/Car Blue.glb")]
    _car_blue: Handle<Gltf>,
    // DeliveryZone
    #[asset(path = "blueprints/DeliveryZone.glb")]
    _delivery_zone: Handle<Gltf>,
    // fence end
    #[asset(path = "blueprints/Fence End.glb")]
    _fence_end: Handle<Gltf>,
    // fence piece
    #[asset(path = "blueprints/Fence Piece.glb")]
    _fence_piece: Handle<Gltf>,
    // fire exit
    #[asset(path = "blueprints/Fire Exit.glb")]
    _fire_exit: Handle<Gltf>,
    // grass
    #[asset(path = "blueprints/Grass.glb")]
    _grass: Handle<Gltf>,
    // mailbox
    #[asset(path = "blueprints/Mailbox.glb")]
    _mailbox: Handle<Gltf>,
    // metal fence
    #[asset(path = "blueprints/Metal Fence.glb")]
    _metal_fence: Handle<Gltf>,
    // npc
    #[asset(path = "blueprints/Npc.glb")]
    _npc: Handle<Gltf>,
    // ramp
    #[asset(path = "blueprints/Ramp.glb")]
    _ramp: Handle<Gltf>,
    // road corner curved
    #[asset(path = "blueprints/Road Corner Curved.glb")]
    _road_corner_curved: Handle<Gltf>,
    // road straight crossing
    #[asset(path = "blueprints/Road Straight Crossing.glb")]
    _road_straight_crossing: Handle<Gltf>,
    // road straight
    #[asset(path = "blueprints/Road Straight.glb")]
    _road_straight: Handle<Gltf>,
    // road t-split
    #[asset(path = "blueprints/Road T-Split.glb")]
    _road_t_split: Handle<Gltf>,
    // sidewalk center
    #[asset(path = "blueprints/Sidewalk Center.glb")]
    _sidewalk_center: Handle<Gltf>,
    // sidewalk corner
    #[asset(path = "blueprints/Sidewalk Corner.glb")]
    _sidewalk_corner: Handle<Gltf>,
    // sidewalk side
    #[asset(path = "blueprints/Sidewalk Side.glb")]
    _sidewalk_side: Handle<Gltf>,
    // traffic light
    #[asset(path = "blueprints/Traffic Light.glb")]
    _traffic_light: Handle<Gltf>,
    // tree
    #[asset(path = "blueprints/Tree.glb")]
    _tree: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct MaterialHandles {
    // Atlas.052
    #[asset(path = "materials/Atlas.052.glb")]
    _atlas_052: Handle<Gltf>,
    // Beige
    #[asset(path = "materials/Beige.glb")]
    _beige: Handle<Gltf>,
    // Bike
    #[asset(path = "materials/Bike.glb")]
    _bike: Handle<Gltf>,
    // Black
    #[asset(path = "materials/Black.glb")]
    _black: Handle<Gltf>,
    // Blue
    #[asset(path = "materials/Blue.glb")]
    _blue: Handle<Gltf>,
    // brick_shade1
    #[asset(path = "materials/brick_shade1.glb")]
    _brick_shade1: Handle<Gltf>,
    // brick_shade2
    #[asset(path = "materials/brick_shade2.glb")]
    _brick_shade2: Handle<Gltf>,
    // brick_shade3
    #[asset(path = "materials/brick_shade3.glb")]
    _brick_shade3: Handle<Gltf>,
    // brick_shadeGreen
    #[asset(path = "materials/brick_shadeGreen.glb")]
    _brick_shade_green: Handle<Gltf>,
    // brick_shadeGreen2
    #[asset(path = "materials/brick_shadeGreen2.glb")]
    _brick_shade_green2: Handle<Gltf>,
    // BrickRed
    #[asset(path = "materials/BrickRed.glb")]
    _brick_red: Handle<Gltf>,
    // Brown
    #[asset(path = "materials/Brown.glb")]
    _brown: Handle<Gltf>,
    // citybits_texture
    #[asset(path = "materials/citybits_texture.glb")]
    _citybits_texture: Handle<Gltf>,
    // concrete
    #[asset(path = "materials/concrete.glb")]
    _concrete: Handle<Gltf>,
    // DarkGrey
    #[asset(path = "materials/DarkGrey.glb")]
    _dark_grey: Handle<Gltf>,
    // door_white
    #[asset(path = "materials/door_white.glb")]
    _door_white: Handle<Gltf>,
    // Dots Stroke
    #[asset(path = "materials/Dots Stroke.glb")]
    _dots_stroke: Handle<Gltf>,
    // Grass
    #[asset(path = "materials/Grass.glb")]
    _grass: Handle<Gltf>,
    // Grey
    #[asset(path = "materials/Grey.glb")]
    _grey: Handle<Gltf>,
    // Headlights
    #[asset(path = "materials/Headlights.glb")]
    _headlights: Handle<Gltf>,
    // LightYellow
    #[asset(path = "materials/LightYellow.glb")]
    _light_yellow: Handle<Gltf>,
    // mailbox
    #[asset(path = "materials/mailbox.glb")]
    _mailbox: Handle<Gltf>,
    // Mat
    #[asset(path = "materials/Mat.glb")]
    _mat: Handle<Gltf>,
    // mat19
    #[asset(path = "materials/mat19.glb")]
    _mat19: Handle<Gltf>,
    // mat20
    #[asset(path = "materials/mat20.glb")]
    _mat20: Handle<Gltf>,
    // metal_fence
    #[asset(path = "materials/metal_fence.glb")]
    _metal_fence: Handle<Gltf>,
    // metal_shade1
    #[asset(path = "materials/metal_shade1.glb")]
    _metal_shade1: Handle<Gltf>,
    // OrangeWallMaterial
    #[asset(path = "materials/OrangeWallMaterial.glb")]
    _orange_wall_material: Handle<Gltf>,
    // RampMaterial
    #[asset(path = "materials/RampMaterial.glb")]
    _ramp_material: Handle<Gltf>,
    // red
    #[asset(path = "materials/red.glb")]
    _red: Handle<Gltf>,
    // sandstone_light
    #[asset(path = "materials/sandstone_light.glb")]
    _sandstone_light: Handle<Gltf>,
    // sandstone
    #[asset(path = "materials/sandstone.glb")]
    _sandstone: Handle<Gltf>,
    // sign_shade2
    #[asset(path = "materials/sign_shade2.glb")]
    _sign_shade2: Handle<Gltf>,
    // sign
    #[asset(path = "materials/sign.glb")]
    _sign: Handle<Gltf>,
    // Silver
    #[asset(path = "materials/Silver.glb")]
    _silver: Handle<Gltf>,
    // TailLights
    #[asset(path = "materials/TailLights.glb")]
    _tail_lights: Handle<Gltf>,
    // Tree
    #[asset(path = "materials/Tree.glb")]
    _tree: Handle<Gltf>,
    // white
    #[asset(path = "materials/white.glb")]
    _white: Handle<Gltf>,
    // window_frame
    #[asset(path = "materials/window_frame.glb")]
    _window_frame: Handle<Gltf>,
    // window_glass
    #[asset(path = "materials/window_glass.glb")]
    _window_glass: Handle<Gltf>,
    // Windows
    #[asset(path = "materials/Windows.glb")]
    _windows: Handle<Gltf>,
    // wood
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
