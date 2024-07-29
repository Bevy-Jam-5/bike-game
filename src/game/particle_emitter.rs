use std::time::Duration;

use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    pbr::NotShadowCaster,
    prelude::*,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub(super) fn plugin<S: ShapeSample<Output = Vec3> + Send + Sync + 'static, M: Material>(
    app: &mut App,
) {
    // Workaround to avoid duplicate systems
    let is_first_instance = !app.world().contains_resource::<IsInitialized>();

    app.register_type::<(ParticleVisuals<M>, ParticleLifetime)>();

    app.insert_resource(RandomSource(ChaCha8Rng::seed_from_u64(4)));

    app.add_systems(Update, spawn_particles::<S, M>);

    if is_first_instance {
        app.add_systems(Update, (tick_particles, despawn_animation).chain());
        app.init_resource::<IsInitialized>();
    }
}

/// `true` if particle systems have been initialized.
#[derive(Resource, Reflect, Clone, Default)]
#[reflect(Resource)]
struct IsInitialized;

/// The source of randomness used.
#[derive(Resource)]
struct RandomSource(ChaCha8Rng);

#[derive(Bundle, Clone)]
pub struct ParticleEmitterBundle<S: ShapeSample + Send + Sync + 'static, M: Material> {
    pub emitter: ParticleEmitter<S>,
    pub particle_visuals: ParticleVisuals<M>,
    pub particle_lifetime: ParticleLifetime,
}

#[derive(Reflect, Clone)]
#[reflect(Component)]
pub struct ParticleEmitter<S: ShapeSample> {
    pub enabled: bool,
    pub shape: S,
    pub spawn_rate: u32,
    pub sampling_mode: SamplingMode,
}

impl<S: ShapeSample + Send + Sync + 'static> Component for ParticleEmitter<S> {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            let Some(spawn_rate) = world
                .get::<ParticleEmitter<S>>(entity)
                .map(|emitter| emitter.spawn_rate)
            else {
                return;
            };

            world
                .commands()
                .entity(entity)
                .insert(EmitterCooldown(Timer::new(
                    Duration::from_secs_f32(1.0 / spawn_rate as f32),
                    TimerMode::Repeating,
                )));
        });
    }
}

/// Component describing the lifetime of particles for a [`ParticleEmitter`].
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct ParticleLifetime {
    pub lifetime_secs: f32,
    pub despawn_animation_secs: f32,
}

/// Component for the visuals of particles for a [`ParticleEmitter`].
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct ParticleVisuals<M: Material> {
    pub mesh: Handle<Mesh>,
    pub material: Handle<M>,
}

/// Mode used for sampling particles on a shape.
#[derive(Reflect, Clone)]
pub enum SamplingMode {
    Interior,
    Boundary,
}

/// Component for spawned particles.
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct Particle {
    progress: f32,
}

/// Component for tracking the despawn animation.
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct DespawnParticle {
    progress: f32,
}

/// Used for tracking how many particles should be spawned at each frame.
#[derive(Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct EmitterCooldown(pub Timer);

#[allow(clippy::too_many_arguments)]
fn spawn_particles<S: ShapeSample<Output = Vec3> + Send + Sync + 'static, M: Material>(
    mut commands: Commands,
    mut random_source: ResMut<RandomSource>,
    mut emitters: Query<(
        Entity,
        &ParticleEmitter<S>,
        &mut EmitterCooldown,
        &ParticleVisuals<M>,
    )>,
    time: Res<Time>,
) {
    let rng = &mut random_source.0;

    for (entity, emitter, mut cooldown, particle_visuals) in emitters
        .iter_mut()
        .filter(|(_, emitter, _, _)| emitter.enabled)
    {
        cooldown.tick(time.delta());

        let spawn_count = cooldown.times_finished_this_tick();

        commands.entity(entity).with_children(|children| {
            for _ in 0..spawn_count {
                let sample = match emitter.sampling_mode {
                    SamplingMode::Interior => emitter.shape.sample_interior(rng),
                    SamplingMode::Boundary => emitter.shape.sample_boundary(rng),
                };

                children.spawn((
                    MaterialMeshBundle {
                        mesh: particle_visuals.mesh.clone(),
                        material: particle_visuals.material.clone(),
                        transform: Transform::from_translation(sample).with_scale(Vec3::ZERO),
                        ..default()
                    },
                    Particle { progress: 0.0 },
                    NotShadowCaster,
                ));
            }
        });
    }
}

fn tick_particles(
    mut commands: Commands,
    time: Res<Time>,
    emitters: Query<(&ParticleLifetime, &Children)>,
    mut samples: Query<(Entity, &mut Transform, &mut Particle), Without<DespawnParticle>>,
) {
    let dt = time.delta_seconds();

    for (particle_lifetime, children) in &emitters {
        let delta_progress = dt / particle_lifetime.lifetime_secs;
        let mut iter = samples.iter_many_mut(children);
        while let Some((entity, mut transform, mut particle)) = iter.fetch_next() {
            particle.progress += delta_progress;
            transform.scale = Vec3::splat(particle.progress.min(1.0));
            if particle.progress >= 1.0 {
                commands
                    .entity(entity)
                    .insert(DespawnParticle { progress: 0.0 });
            }
        }
    }
}

fn despawn_animation(
    mut commands: Commands,
    time: Res<Time>,
    emitters: Query<(&ParticleLifetime, &Children)>,
    mut samples: Query<(Entity, &mut Transform, &mut DespawnParticle)>,
) {
    let dt = time.delta_seconds();

    for (particle_lifetime, children) in &emitters {
        let delta_progress = dt / particle_lifetime.despawn_animation_secs;
        let mut iter = samples.iter_many_mut(children);
        while let Some((entity, mut transform, mut point)) = iter.fetch_next() {
            point.progress += delta_progress;
            transform.scale = Vec3::splat(1.0 - point.progress.min(1.0));
            if point.progress >= 1.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
