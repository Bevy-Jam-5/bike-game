use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::assets::{AudioSfxHandles, SfxKey};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<AudioSfxHandles>,
    q_audio: Query<&SfxKey>,
) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
    };
    if q_audio.iter().any(|&key| key == sfx_key) {
        return;
    }
    let source = match sfx_key {
        SfxKey::ButtonHover => &sfx_handles.button_hover,
        SfxKey::ButtonPress => &sfx_handles.button_press,
        SfxKey::Cash => &sfx_handles.cash,
        SfxKey::Awesome => &sfx_handles.awesome,
        SfxKey::Yeet => &sfx_handles.yeet,
    }
    .clone_weak();
    commands.spawn((
        sfx_key,
        AudioSourceBundle {
            source,
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
        },
    ));
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
}
