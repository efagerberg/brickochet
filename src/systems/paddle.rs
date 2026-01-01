use crate::components::{ball, paddle};
use bevy::input::mouse;
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<mouse::MouseMotion>,
    paddle: Single<&mut Transform, (With<paddle::Paddle>, Without<ball::Ball>)>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.025;
    let mut paddle_transform = paddle.into_inner();

    paddle_transform.translation.x += delta.x * sensitivity;
    paddle_transform.translation.y -= delta.y * sensitivity; // invert Y if needed
}

pub fn paddle_ball_collision(
    ball: Single<(&mut Transform, &mut ball::Velocity), With<ball::Ball>>,
    paddle: Single<(&Transform, &paddle::PaddleSize), (With<paddle::Paddle>, Without<ball::Ball>)>,
) {
    let (mut ball_transform, mut ball_velocity) = ball.into_inner();
    let (paddle_transform, paddle_size) = paddle.into_inner();

    let p = paddle_transform.translation;

    let b = ball_transform.translation;

    // 1. Must be moving toward the paddle (+Z example)
    if ball_velocity.0.z <= 0.0 {
        return;
    }

    // 2. X overlap
    if (b.x - p.x).abs() > paddle_size.half_width + ball::BALL_RADIUS {
        return;
    }

    // 3. Y overlap
    if (b.y - p.y).abs() > paddle_size.half_height + ball::BALL_RADIUS {
        return;
    }

    // 4. Z overlap band
    let z_min = p.z - paddle_size.contact_depth - ball::BALL_RADIUS;
    let z_max = p.z + paddle_size.contact_depth + ball::BALL_RADIUS;

    if b.z < z_min || b.z > z_max {
        return;
    }

    // --- Collision confirmed ---

    // Clamp ball to paddle surface
    ball_transform.translation.z = z_min;

    // Reflect Z only
    ball_velocity.0.z = -ball_velocity.0.z;
}
