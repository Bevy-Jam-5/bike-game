use std::f32::consts::PI;
use std::iter;

use crate::AppSet;
use crate::{game::materials::SingleColorMaterial, util::single};
use crate::{game::spawn::player::Player, third_party::avian::DisableSensor};
use avian3d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

use super::{
    advance_quest::{ActiveQuest, AdvanceQuest},
    quest_place::QuestPlace,
};
use crate::game::particle_emitter::{
    ParticleEmitter, ParticleEmitterBundle, ParticleLifetime, ParticleVisuals, SamplingMode,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_create_delivery_zone);
    app.register_type::<(DeliveryZone, DeliveryZoneLink, DeliveryZoneParticleVisuals)>();
    app.add_systems(Startup, init_particle_visuals);
    app.add_systems(
        Update,
        (
            on_delivery_player_collision.in_set(AppSet::ReadCollisions),
            control_emitters,
        )
            .chain(),
    );
}

/// Marker for a delivery zone for quest advancement.
/// Created automatically by inserting a [`QuestPlace`] on an entity.
#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub struct DeliveryZone;

/// Link on [`QuestPlace`] pointing to the delivery zone entity holding the actual collider.
#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Debug, Component, PartialEq)]
pub struct DeliveryZoneLink(pub Entity);

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct DeliveryZoneParticleVisuals(pub ParticleVisuals<SingleColorMaterial>);

const COLOR: Srgba = tailwind::AMBER_200;
const INTENSITY: f32 = 200_000.0;

fn init_particle_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SingleColorMaterial>>,
) {
    commands.insert_resource(DeliveryZoneParticleVisuals(ParticleVisuals {
        mesh: meshes.add(Sphere::new(0.06).mesh().ico(1).unwrap()),
        material: materials.add(Color::from(COLOR)),
    }));
}

/// Add [`DeliveryZoneLink`] to the [`QuestPlace`] entity.
/// Not a hook because `iter_ancestors` does not work there :(
fn on_create_delivery_zone(
    trigger: Trigger<OnAdd, DeliveryZone>,
    mut commands: Commands,
    q_parent: Query<&Parent>,
    q_place: Query<Entity, With<QuestPlace>>,
    particle_visuals: Res<DeliveryZoneParticleVisuals>,
) {
    let entity = trigger.entity();
    let place_entity = iter::once(entity)
        .chain(q_parent.iter_ancestors(entity))
        .find(|&e| q_place.contains(e));
    let Some(place_entity) = place_entity else {
        error!("Failed to get place of delivery zone entity.");
        return;
    };
    commands
        .entity(place_entity)
        .insert(DeliveryZoneLink(entity))
        .with_children(|children| {
            children.spawn((
                ParticleEmitterBundle {
                    emitter: ParticleEmitter {
                        enabled: false,
                        shape: Extrusion::new(Annulus::new(2.75, 3.0), 0.5),
                        spawn_rate: 300,
                        sampling_mode: SamplingMode::Interior,
                    },
                    particle_lifetime: ParticleLifetime {
                        lifetime_secs: 0.5,
                        despawn_animation_secs: 0.5,
                    },
                    particle_visuals: particle_visuals.0.clone(),
                },
                PointLightBundle {
                    point_light: PointLight {
                        color: Color::from(COLOR),
                        intensity: 0.0,
                        range: 3.0,
                        radius: 0.0,
                        shadows_enabled: false,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.25, 0.0)
                        .with_rotation(Quat::from_rotation_x(PI / 2.0)),
                    ..default()
                },
            ));
        });
}

fn on_delivery_player_collision(
    mut commands: Commands,
    q_delivery_zone: Query<
        (Entity, &CollidingEntities),
        (With<DeliveryZone>, Without<DisableSensor>),
    >,
    q_parent: Query<&Parent>,
    q_place: Query<Entity, With<QuestPlace>>,
    q_player: Query<Entity, With<Player>>,
) {
    for (entity, collisions) in q_delivery_zone.iter() {
        for &collision_entity in collisions.iter() {
            if !q_player.contains(collision_entity) {
                // In theory, only players can collide with delivery zones
                // because of the way collision layers are set up, so this should never happen.
                // But let's be safe to not break the game in the worst case.
                error!("Non-player entity collided with delivery zone.");
                continue;
            }

            let Some(place_entity) = q_parent
                .iter_ancestors(entity)
                .find_map(|e| q_place.get(e).ok())
            else {
                error!("Failed to get place of delivery zone entity.");
                return;
            };
            info!("collision");
            commands.trigger_targets(AdvanceQuest, place_entity);
        }
    }
}

fn control_emitters(
    time: Res<Time>,
    q_player: Query<&Transform, With<Player>>,
    q_delivery_zone: Query<(Entity, &DeliveryZoneLink, &GlobalTransform)>,
    q_disabled_collider: Query<(), With<DisableSensor>>,
    children: Query<&Children>,
    mut emitters: Query<(&mut PointLight, &mut ParticleEmitter<Extrusion<Annulus>>)>,
    quest: Option<Res<ActiveQuest>>,
) {
    let dt = time.delta_seconds();
    let player_transform = single!(q_player);
    for (entity, link, zone_transform) in &q_delivery_zone {
        let distance_sq = player_transform
            .translation
            .distance_squared(zone_transform.translation());
        let cut_off = if quest.is_some() { 40.0 } else { 20.0 };
        let is_too_far = distance_sq > cut_off * cut_off;
        let is_disabled = q_disabled_collider.contains(link.0) || is_too_far;
        let mut iter =
            emitters.iter_many_mut(iter::once(entity).chain(children.iter_descendants(entity)));
        while let Some((mut light, mut emitter)) = iter.fetch_next() {
            emitter.enabled = !is_disabled;
            let target = if is_disabled { 0.0 } else { INTENSITY };
            let decay_rate = f32::ln(10.0);
            light.intensity = light
                .intensity
                .lerp(target, 1.0 - f32::exp(-decay_rate * dt));
        }
    }
}
