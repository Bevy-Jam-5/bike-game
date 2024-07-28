use bevy::{prelude::*, window::CursorGrabMode};

use crate::{screen::PlayState, util::single_mut};

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "dev"))]
    {
        app.add_systems(Update, capture_cursor.run_if(in_state(PlayState::Active)));
    }
    app.add_systems(OnExit(PlayState::Active), release_cursor);
}

fn capture_cursor(
    mut q_window: Query<&mut Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Clear Bevy's grab mode cache by setting a different grab mode
        // because an unlocked cursor will not update the current `CursorGrabMode`.
        // See <https://github.com/bevyengine/bevy/issues/8949>
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }
}

fn release_cursor(mut q_window: Query<&mut Window>) {
    let mut window = single_mut!(q_window);
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
