use bevy::prelude::*;

use crate::resources::input;
use crate::components::paddle;

pub fn paddle_mouse_control(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_position: Res<input::MousePosition>,
    mut query: Query<&mut Transform, With<paddle::Paddle>>,
) {
    let (camera, camera_transform) = camera_query.single().unwrap();

    if let Some(screen_pos) = mouse_position.0 {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, screen_pos) {
            // Intersect ray with Z = 0 plane (your playfield)
            let world_target = ray.origin + ray.direction * (-ray.origin.z / ray.direction.z);

            for mut transform in &mut query {
                transform.translation.x = world_target.x;
                transform.translation.y = world_target.y;
            }
        }
    }
}
