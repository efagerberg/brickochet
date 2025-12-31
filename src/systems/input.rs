use bevy::prelude::*;
use crate::resources;

pub fn update_mouse_position(
    mut cursor_moved_messages: MessageReader<CursorMoved>,
    mut mouse_pos: ResMut<resources::input::MousePosition>,
) {
    for event in cursor_moved_messages.read() {
        mouse_pos.0 = Some(event.position);
    }
}
