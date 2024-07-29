use crate::{game::money::GainMoney, screen::PlayState, util::single};
use bevy::{prelude::*, time::Stopwatch};
use bevy_tnua::prelude::*;

use super::assets::FontHandles;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<StuntStopwatch>();
    app.register_type::<StuntStopwatch>();
    app.add_systems(
        Update,
        (tick_stopwatch, reset_stopwatch, spawn_ui, update_ui)
            .chain()
            .run_if(in_state(PlayState::Active)),
    );
    app.add_systems(OnExit(PlayState::Active), clear_stopwatch);
    app.observe(on_stunt);
}

#[derive(Debug, Reflect, Resource, Default)]
#[reflect(Debug, Resource, Default)]
pub struct StuntStopwatch {
    air: Stopwatch,
    ground: Stopwatch,
}

fn tick_stopwatch(
    q_controller: Query<&TnuaController>,
    mut stopwatch: ResMut<StuntStopwatch>,
    time: Res<Time>,
) {
    let controller = single!(q_controller);
    if controller.is_airborne().is_ok_and(|airborne| airborne) {
        stopwatch.air.tick(time.delta());
    } else {
        stopwatch.ground.tick(time.delta());
    }
}

fn spawn_ui(
    mut commands: Commands,
    stopwatch: Res<StuntStopwatch>,
    q_stunt_ui: Query<(), With<StuntUiRoot>>,
    fonts: Res<FontHandles>,
) {
    let air = stopwatch.air.elapsed().as_secs_f64();
    if air > MIN_AIR && q_stunt_ui.is_empty() {
        use Val::*;

        let text_style = TextStyle {
            font: fonts.rubik_regular.clone_weak(),
            font_size: 24.0,
            color: Color::WHITE,
        };
        commands
            .spawn((
                StuntUiRoot,
                StateScoped(PlayState::Active),
                NodeBundle {
                    background_color: Color::srgba(0.0, 0.0, 0.1, 0.7).into(),
                    style: Style {
                        padding: UiRect::all(Px(10.0)),
                        top: Px(40.0),
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
                    border_radius: BorderRadius::bottom_right(Px(10.0)),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("Stunt text"),
                    StuntUiText,
                    TextBundle::from_sections([TextSection::new("", text_style.clone())]),
                ));
            });
    }
}

fn reset_stopwatch(mut stopwatch: ResMut<StuntStopwatch>, mut commands: Commands) {
    const CUTOFF: f64 = 0.1;
    let air = stopwatch.air.elapsed().as_secs_f64();
    let ground = stopwatch.ground.elapsed().as_secs_f64();
    if ground > CUTOFF {
        stopwatch.ground.reset();
        stopwatch.air.reset();
        commands.trigger(StuntEvent { air, ground });
    }
}

fn clear_stopwatch(mut stopwatch: ResMut<StuntStopwatch>) {
    stopwatch.air.reset();
    stopwatch.ground.reset();
}

fn update_ui(stopwatch: Res<StuntStopwatch>, mut q_text: Query<&mut Text, With<StuntUiText>>) {
    let air = stopwatch.air.elapsed().as_secs_f64();
    for mut text in q_text.iter_mut() {
        let label = format!("Stunt time: {:.2} s", air);
        text.sections[0].value = label;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Event)]
struct StuntEvent {
    air: f64,
    ground: f64,
}

const MIN_AIR: f64 = 0.5;

fn on_stunt(
    trigger: Trigger<StuntEvent>,
    mut commands: Commands,
    q_text: Query<Entity, With<StuntUiRoot>>,
) {
    const MONEY_PER_SECOND: f64 = 0.5;
    let event = trigger.event();
    for entity in q_text.iter() {
        commands.entity(entity).despawn_recursive();
    }
    if event.air > MIN_AIR {
        let pay = event.air * MONEY_PER_SECOND;
        // round to two decimal places in steps of 0.05
        let pay = (pay * 20.0).round() / 20.0;
        commands.trigger(GainMoney(pay as f32));
    }
}

#[derive(Debug, Reflect, Component, Default)]
#[reflect(Debug, Component, Default)]
pub struct StuntUiRoot;

#[derive(Debug, Reflect, Component, Default)]
#[reflect(Debug, Component, Default)]
pub struct StuntUiText;
