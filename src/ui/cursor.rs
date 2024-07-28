use bevy::{prelude::*, window::CursorGrabMode};

use crate::{screen::PlayState, util::single_mut};

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "dev"))]
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_systems(OnEnter(PlayState::Active), capture_cursor);
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Running this continuously due to https://github.com/TheBevyFlock/bevy_quickstart/issues/198
            app.add_systems(Update, capture_cursor.run_if(in_state(PlayState::Active)));
        }
    }
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
