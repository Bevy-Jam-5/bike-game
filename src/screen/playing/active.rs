use super::PlayState;
use crate::{
    game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::hud::SpawnHud},
    util::single_mut,
};
use bevy::prelude::*;
use blenvy::GameWorldTag;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PlayState::Active), enter_active);
}

fn enter_active(mut commands: Commands, mut q_world: Query<&mut Visibility, With<GameWorldTag>>) {
    commands.trigger(SpawnHud);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    let mut visibility = single_mut!(q_world);
    *visibility = Visibility::Inherited;
}
