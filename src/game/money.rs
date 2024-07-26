use bevy::prelude::*;

use crate::{screen::Screen, util::single_mut};

use super::{
    quest::{finish_quest::FinishQuest, quest_place::QuestPlace},
    spawn::hud::MoneyText,
};

pub fn plugin(app: &mut App) {
    app.init_resource::<Money>().register_type::<Money>();

    app.add_systems(Update, update_money_text.run_if(in_state(Screen::Playing)));
    app.observe(on_finish_quest);

    // Leaving the gameplay screen currently resets the world, so reset the money.
    app.add_systems(OnExit(Screen::Playing), reset_money);
}

#[derive(Debug, Resource, Clone, Default, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct Money(pub f32);

fn update_money_text(money: Res<Money>, mut money_text: Query<&mut Text, With<MoneyText>>) {
    if !money.is_changed() {
        return;
    }

    let mut text = single_mut!(money_text);
    text.sections[1].value = format!("${}", money.0);
}

fn on_finish_quest(trigger: Trigger<FinishQuest>, mut money: ResMut<Money>) {
    let pay = match trigger.event().0 {
        QuestPlace::PizzaNpc => 5.0,
        QuestPlace::PostOffice => 3.0,
        _ => 0.0,
    };
    money.0 += pay;
    info!("Received ${pay}");
}

fn reset_money(mut money: ResMut<Money>) {
    money.0 = 0.0;
}
