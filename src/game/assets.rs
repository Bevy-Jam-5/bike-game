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
    // bus stop sign
    #[asset(path = "blueprints/Bus Stop Sign.glb")]
    _bus_stop_sign: Handle<Gltf>,
    // bus stop
    #[asset(path = "blueprints/Bus Stop.glb")]
    // bush
    #[asset(path = "blueprints/Bush.glb")]
    _bush: Handle<Gltf>,
    // car blue
    #[asset(path = "blueprints/Car Blue.glb")]
    _car_blue: Handle<Gltf>,
    // cone
    #[asset(path = "blueprints/Cone.glb")]
    _cone: Handle<Gltf>,
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
    // Npc Mail
    #[asset(path = "blueprints/Npc Mail.glb")]
    _npc_mail: Handle<Gltf>,
    // Npc Pizza
    #[asset(path = "blueprints/Npc Pizza.glb")]
    _npc_pizza: Handle<Gltf>,
    // overpass block
    #[asset(path = "blueprints/Overpass Block.glb")]
    _overpass_block: Handle<Gltf>,
    // overpass tunnel
    #[asset(path = "blueprints/Overpass Tunnel.glb")]
    _overpass_tunnel: Handle<Gltf>,
    // poster
    #[asset(path = "blueprints/Poster.glb")]
    _poster: Handle<Gltf>,
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
    // roof
    #[asset(path = "blueprints/Roof.glb")]
    _roof: Handle<Gltf>,
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
    // trash bag
    #[asset(path = "blueprints/Trash Bag.glb")]
    _trash_bag: Handle<Gltf>,
    // trash container
    #[asset(path = "blueprints/Trash Container.glb")]
    _trash_container: Handle<Gltf>,
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
    // BlueBusStop
    #[asset(path = "materials/BlueBusStop.glb")]
    _blue_bus_stop: Handle<Gltf>,
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
    // Dark_gray
    #[asset(path = "materials/Dark_gray.glb")]
    _dark_gray: Handle<Gltf>,
    // DarkGrey
    #[asset(path = "materials/DarkGrey.glb")]
    _dark_grey: Handle<Gltf>,
    // door_white
    #[asset(path = "materials/door_white.glb")]
    _door_white: Handle<Gltf>,
    // Dots Stroke
    #[asset(path = "materials/Dots Stroke.glb")]
    _dots_stroke: Handle<Gltf>,
    // Eyes
    #[asset(path = "materials/Eyes.glb")]
    _eyes: Handle<Gltf>,
    // Grass
    #[asset(path = "materials/Grass.glb")]
    _grass: Handle<Gltf>,
    // Green
    #[asset(path = "materials/Green.glb")]
    _green: Handle<Gltf>,
    // Grey
    #[asset(path = "materials/Grey.glb")]
    _grey: Handle<Gltf>,
    // Hair
    #[asset(path = "materials/Hair.glb")]
    _hair: Handle<Gltf>,
    // Headlights
    #[asset(path = "materials/Headlights.glb")]
    _headlights: Handle<Gltf>,
    // LightBlue_BusStop
    #[asset(path = "materials/LightBlue_BusStop.glb")]
    _light_blue_bus_stop: Handle<Gltf>,
    // LightYellow
    #[asset(path = "materials/LightYellow.glb")]
    _light_yellow: Handle<Gltf>,
    // mailbox
    #[asset(path = "materials/mailbox.glb")]
    _mailbox: Handle<Gltf>,
    // Mat
    #[asset(path = "materials/Mat.glb")]
    _mat: Handle<Gltf>,
    // mat17
    #[asset(path = "materials/mat17.glb")]
    _mat17: Handle<Gltf>,
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
    // Orang
    #[asset(path = "materials/Orang.glb")]
    _orang: Handle<Gltf>,
    // Orange
    #[asset(path = "materials/Orange.glb")]
    _orange: Handle<Gltf>,
    // Pants
    #[asset(path = "materials/Pants.glb")]
    _pants: Handle<Gltf>,
    // poster
    #[asset(path = "materials/poster.glb")]
    _poster: Handle<Gltf>,
    // red
    #[asset(path = "materials/red.glb")]
    _red: Handle<Gltf>,
    // sandstone_light
    #[asset(path = "materials/sandstone_light.glb")]
    _sandstone_light: Handle<Gltf>,
    // sandstone
    #[asset(path = "materials/sandstone.glb")]
    _sandstone: Handle<Gltf>,
    // Shirt
    #[asset(path = "materials/Shirt.glb")]
    _shirt: Handle<Gltf>,
    // Shoes
    #[asset(path = "materials/Shoes.glb")]
    _shoes: Handle<Gltf>,
    // Sidewalk
    #[asset(path = "materials/Sidewalk.glb")]
    _sidewalk: Handle<Gltf>,
    // sign_shade2
    #[asset(path = "materials/sign_shade2.glb")]
    _sign_shade2: Handle<Gltf>,
    // sign
    #[asset(path = "materials/sign.glb")]
    _sign: Handle<Gltf>,
    // Silver
    #[asset(path = "materials/Silver.glb")]
    _silver: Handle<Gltf>,
    // Skin
    #[asset(path = "materials/Skin.glb")]
    _skin: Handle<Gltf>,
    // Socks
    #[asset(path = "materials/Socks.glb")]
    _socks: Handle<Gltf>,
    // TailLights
    #[asset(path = "materials/TailLights.glb")]
    _tail_lights: Handle<Gltf>,
    // Tree
    #[asset(path = "materials/Tree.glb")]
    _tree: Handle<Gltf>,
    // White
    #[asset(path = "materials/White.glb")]
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
    // Woman
    #[asset(path = "materials/Woman.glb")]
    _woman: Handle<Gltf>,
    // wood
    #[asset(path = "materials/wood.glb")]
    _wood: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageHandles {
    #[asset(path = "images/skybox.png")]
    pub skybox: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioSfxHandles {
    #[asset(path = "audio/sfx/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/button_press.ogg")]
    pub button_press: Handle<AudioSource>,
    #[asset(path = "audio/sfx/cash.mp3")]
    pub cash: Handle<AudioSource>,
    #[asset(path = "audio/sfx/awesome.mp3")]
    pub awesome: Handle<AudioSource>,
    #[asset(path = "audio/sfx/yeet.mp3")]
    pub yeet: Handle<AudioSource>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect, Component)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Cash,
    Awesome,
    Yeet,
}

#[derive(AssetCollection, Resource)]
pub struct AudioSoundtrackHandles {
    #[asset(path = "audio/soundtracks/ingame.mp3")]
    pub gameplay: Handle<AudioSource>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

#[derive(AssetCollection, Resource)]
pub struct FontHandles {
    #[asset(path = "fonts/Rubik-Regular.ttf")]
    pub rubik_regular: Handle<Font>,
    #[asset(path = "fonts/Rubik-Bold.ttf")]
    pub rubik_bold: Handle<Font>,
}
