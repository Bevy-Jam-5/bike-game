use bevy::prelude::*;

use crate::game::{assets::SfxKey, audio::sfx::PlaySfx, quest::advance_quest::ActiveQuest};
use crate::{screen::Screen, util::single_mut};

use super::{
    game_end::GAME_END_MONEY,
    quest::{finish_quest::FinishQuest, quest_place::QuestPlace},
    spawn::hud::MoneyText,
};

pub fn plugin(app: &mut App) {
    app.init_resource::<Money>().register_type::<Money>();

    app.add_systems(
        Update,
        update_money_text
            .run_if(resource_changed::<Money>)
            .run_if(in_state(Screen::Playing)),
    );
    app.observe(on_finish_quest);
    app.observe(on_gain_money);

    // Leaving the gameplay screen currently resets the world, so reset the money.
    app.add_systems(OnExit(Screen::Playing), reset_money);
}

#[derive(Debug, Resource, Clone, Default, PartialEq, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct Money(pub f32);

fn update_money_text(money: Res<Money>, mut money_text: Query<&mut Text, With<MoneyText>>) {
    let mut text = single_mut!(money_text);
    let total = GAME_END_MONEY;
    text.sections[1].value = format!("${} out of ${total}", money.0);
}

fn on_finish_quest(
    _trigger: Trigger<FinishQuest>,
    mut commands: Commands,
    mut active_quest: Option<ResMut<ActiveQuest>>,
) {
    let Some(active_quest) = active_quest.as_mut() else {
        error!("Cannot finish quest without active quest.");
        return;
    };

    let quest_giver = active_quest.quest_giver().unwrap();
    let pay = match quest_giver.place {
        QuestPlace::PizzaNpc => 5.0,
        QuestPlace::MailNpc => 3.0,
        _ => 0.0,
    };
    commands.trigger(GainMoney(pay));
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Event, Deref, DerefMut)]
#[reflect(Debug, PartialEq)]
pub struct GainMoney(pub f32);

fn on_gain_money(trigger: Trigger<GainMoney>, mut money: ResMut<Money>, mut commands: Commands) {
    let pay = trigger.event().0;
    money.0 += pay;
    info!("Received ${pay}");
    commands.trigger(PlaySfx::Key(SfxKey::Cash));
}

fn reset_money(mut money: ResMut<Money>) {
    money.0 = 0.0;
}
