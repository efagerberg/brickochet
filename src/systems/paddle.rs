use bevy::prelude::*;
use bevy::window;

use crate::components::paddle;

pub fn paddle_mouse_control(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut paddle_query: Query<&mut Transform, With<paddle::Paddle>>,
    windows: Query<&Window, With<window::PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_query.single().unwrap();

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };

    // Paddle plane (Z = 5.0)
    let t = (5.0 - ray.origin.z) / ray.direction.z;
    let world_pos = ray.origin + ray.direction * t;

    for mut transform in &mut paddle_query {
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}
