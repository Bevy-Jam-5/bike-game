use bevy::prelude::*;

use crate::util::single_mut;
use crate::{third_party::leafwing_input_manager::PlayerAction, util::single};
use leafwing_input_manager::{plugin::InputManagerSystem, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(LastPedal, PedalTimer)>();
    app.add_event::<PedalEvent>();
    app.add_systems(
        PreUpdate,
        update_pedal.after(InputManagerSystem::ManualControl),
    );
    app.add_systems(FixedPostUpdate, on_pedal);
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq, Default)]
#[reflect(Debug, Component, PartialEq, Default)]
pub enum LastPedal {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Debug, Component, Clone, Reflect, Deref, DerefMut, Default)]
#[reflect(Debug, Component, Default)]
pub struct PedalTimer(Timer);

#[derive(Debug, Event, Clone, Reflect, Default)]
#[reflect(Debug, Default)]
pub struct PedalEvent;

fn update_pedal(
    time: Res<Time>,
    mut q_player: Query<(&ActionState<PlayerAction>, &mut PedalTimer, &mut LastPedal)>,
    mut pedal_events: EventWriter<PedalEvent>,
) {
    let (action, mut timer, mut last_pedal) = single_mut!(q_player);
    timer.tick(time.delta());
    if timer.finished() {
        *last_pedal = LastPedal::None;
    }
    const PEDAL_SECS: f32 = 1.0;

    let pedalled = if action.just_pressed(&PlayerAction::PedalLeft)
        && matches!(*last_pedal, LastPedal::Right | LastPedal::None)
    {
        *last_pedal = LastPedal::Left;
        true
    } else if action.just_pressed(&PlayerAction::PedalRight)
        && matches!(*last_pedal, LastPedal::Left | LastPedal::None)
    {
        *last_pedal = LastPedal::Right;
        true
    } else {
        false
    };

    if !pedalled {
        return;
    }

    timer.0 = Timer::from_seconds(PEDAL_SECS, TimerMode::Once);
    pedal_events.send(PedalEvent);
}

fn on_pedal(mut pedal_events: EventReader<PedalEvent>, q_player: Query<&LastPedal>) {
    let last_pedal = single!(q_player);
    for _ in pedal_events.read() {
        info!("New pedal: {:?}", last_pedal);
    }
}
