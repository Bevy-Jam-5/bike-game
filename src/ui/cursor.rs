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

pub fn capture_cursor(mut q_window: Query<&mut Window>, mut last: Local<CursorGrabMode>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = false;
    #[cfg(not(target_arch = "wasm32"))]
    {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        *last = CursorGrabMode::Locked;
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Confined actually resets the mouse position,
        // but only works when paired with a click.
        // So we run this every frame.
        // BUT: Bevy is being very smart and caches the last grab mode,
        // so we need to change it every frame to trip it up.
        // Very very cool. Nice code. 10/10.
        let mode = match *last {
            CursorGrabMode::None => CursorGrabMode::Confined,
            CursorGrabMode::Confined => CursorGrabMode::Locked,
            CursorGrabMode::Locked => CursorGrabMode::Confined,
        };
        window.cursor.grab_mode = mode;
        *last = mode;
    }
}

pub fn release_cursor(mut q_window: Query<&mut Window>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
