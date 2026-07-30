pub fn grab_cursor(cursor_options: &mut bevy::window::CursorOptions) {
    cursor_options.visible = false;
    cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
}

pub fn release_cursor(cursor_options: &mut bevy::window::CursorOptions) {
    cursor_options.visible = true;
    cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
}
