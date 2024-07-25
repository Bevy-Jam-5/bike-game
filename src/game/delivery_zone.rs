use avian3d::prelude::*;
use bevy::prelude::*;

use crate::game::spawn::player::Player;
use crate::util::single;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DeliveryZone>();
    app.add_systems(Update, on_delivery_player_collision);
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
struct DeliveryZone;

fn on_delivery_player_collision(
    q_delivery_zone: Query<&CollidingEntities, With<DeliveryZone>>,
    q_player: Query<Entity, With<Player>>,
) {
    let player = single!(q_player);
    for delivery_zone in q_delivery_zone.iter() {
        if delivery_zone.0.contains(&player) {
            info!("Player entered delivery zone");
        }
    }
}
