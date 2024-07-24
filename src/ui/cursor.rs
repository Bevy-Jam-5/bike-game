use bevy::{prelude::*, window::CursorGrabMode};

use crate::{screen::Screen, util::single_mut};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), capture_cursor);
    app.add_systems(OnExit(Screen::Playing), release_cursor);
}

fn capture_cursor(mut q_window: Query<&mut Window>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn release_cursor(mut q_window: Query<&mut Window>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
