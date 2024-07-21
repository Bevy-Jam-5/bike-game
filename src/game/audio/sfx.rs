use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

use crate::game::assets::{AudioSfxHandles, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(trigger: Trigger<PlaySfx>, mut commands: Commands, sfx_handles: Res<AudioSfxHandles>) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
        PlaySfx::RandomStep => random_step(),
    };
    let source = match sfx_key {
        SfxKey::ButtonHover => &sfx_handles.button_hover,
        SfxKey::ButtonPress => &sfx_handles.button_press,
        SfxKey::Step1 => &sfx_handles.step1,
        SfxKey::Step2 => &sfx_handles.step2,
        SfxKey::Step3 => &sfx_handles.step3,
        SfxKey::Step4 => &sfx_handles.step4,
    }
    .clone_weak();
    commands.spawn(AudioSourceBundle {
        source,
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    RandomStep,
}

fn random_step() -> SfxKey {
    [SfxKey::Step1, SfxKey::Step2, SfxKey::Step3, SfxKey::Step4]
        .choose(&mut rand::thread_rng())
        .copied()
        .unwrap()
}
