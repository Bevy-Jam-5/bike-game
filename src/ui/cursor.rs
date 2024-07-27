use bevy::{prelude::*, window::CursorGrabMode};

use crate::{screen::PlayState, util::single_mut};

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "dev"))]
    app.add_systems(OnEnter(PlayState::Active), capture_cursor);
    app.add_systems(OnExit(PlayState::Active), release_cursor);
}

pub fn capture_cursor(mut q_window: Query<&mut Window>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn release_cursor(mut q_window: Query<&mut Window>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
