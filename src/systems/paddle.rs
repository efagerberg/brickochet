use crate::components::{ball, paddle};
use bevy::input::mouse;
use bevy::prelude::*;

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

pub fn paddle_ball_collision(
    mut ball_query: Query<(&mut Transform, &mut ball::Velocity), With<ball::Ball>>,
    paddle_query: Query<(&Transform, &paddle::PaddleSize), (With<paddle::Paddle>, Without<ball::Ball>)>,
) {
    for (paddle_transform, paddle_size) in &paddle_query {
        let p = paddle_transform.translation;

        for (mut ball_transform, mut velocity) in &mut ball_query {
            let b = ball_transform.translation;

            // 1. Must be moving toward the paddle (+Z example)
            if velocity.0.z <= 0.0 {
                continue;
            }

            // 2. X overlap
            if (b.x - p.x).abs() > paddle_size.half_width + ball::BALL_RADIUS {
                continue;
            }

            // 3. Y overlap
            if (b.y - p.y).abs() > paddle_size.half_height + ball::BALL_RADIUS {
                continue;
            }

            // 4. Z overlap band
            let z_min = p.z - paddle_size.contact_depth - ball::BALL_RADIUS;
            let z_max = p.z + paddle_size.contact_depth + ball::BALL_RADIUS;

            if b.z < z_min || b.z > z_max {
                continue;
            }

            // --- Collision confirmed ---

            // Clamp ball to paddle surface
            ball_transform.translation.z = z_min;

            // Reflect Z only
            velocity.0.z = -velocity.0.z;
        }
    }
}
