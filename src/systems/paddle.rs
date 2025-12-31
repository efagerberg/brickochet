use bevy::prelude::*;
use bevy::input::mouse;
use crate::components::paddle;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<mouse::MouseMotion>,
    mut paddle_query: Query<&mut Transform, With<paddle::Paddle>>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.025;

    for mut transform in &mut paddle_query {
        transform.translation.x += delta.x * sensitivity;
        transform.translation.y -= delta.y * sensitivity; // invert Y if needed
    }
}
