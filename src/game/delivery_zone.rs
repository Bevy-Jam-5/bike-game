use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DeliveryZone>();
    app.add_systems(Update, on_delivery_player_collision);
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
struct DeliveryZone;

fn on_delivery_player_collision(q_delivery_zone: Query<&CollidingEntities, With<DeliveryZone>>) {}
