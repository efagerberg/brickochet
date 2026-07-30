use bevy::prelude::*;

use crate::input::utils;

pub fn grab_cursor(mut cursor_options: Single<&mut bevy::window::CursorOptions>) {
    utils::grab_cursor(&mut cursor_options);
}

pub fn release_cursor(mut cursor_options: Single<&mut bevy::window::CursorOptions>) {
    utils::release_cursor(&mut cursor_options);
}

pub fn update_cursor_on_mouse_input(
    mut cursor_options: Single<&mut bevy::window::CursorOptions>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        utils::grab_cursor(&mut cursor_options);
    }

    if key.just_pressed(KeyCode::Escape) {
        utils::release_cursor(&mut cursor_options);
    }
}
