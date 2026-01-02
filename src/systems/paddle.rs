use crate::components::paddle::PaddleDelta;
use crate::components::{ball, paddle, physics};
use bevy::input::mouse;
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<mouse::MouseMotion>,
    paddle: Single<(&mut Transform, &mut PaddleDelta), (With<paddle::Paddle>, Without<ball::Ball>)>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.025;
    let (mut paddle_transform, mut paddle_delta) = paddle.into_inner();
    paddle_delta.0 = Vec2::ZERO;
    let new_velocity = delta * sensitivity;

    paddle_transform.translation.x += new_velocity.x;
    paddle_transform.translation.y -= new_velocity.y; // invert Y if needed
    paddle_delta.0 = delta;
}

pub fn paddle_ball_collision(
    ball: Single<(&mut Transform, &mut physics::Velocity, &mut physics::Curve), With<ball::Ball>>,
    paddle: Single<
        (&Transform, &paddle::PaddleSize, &paddle::PaddleDelta),
        (With<paddle::Paddle>, Without<ball::Ball>)
    >,
) {
    let (mut ball_transform, mut ball_velocity, mut curve) = ball.into_inner();
    let (paddle_transform, paddle_size, paddle_velocity_delta) = paddle.into_inner();

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

    set_curve_on_collision(paddle_velocity_delta.0, &mut curve);
}


pub fn set_curve_on_collision(
    paddle_delta: Vec2,
    curve: &mut physics::Curve,
) {
    const REGULAR: f32 = 0.15;
    const SUPER: f32 = 0.3;

    curve.0.x = match paddle_delta.x {
        d if d <= -10.0 => SUPER,
        d if d <= -5.0  => REGULAR,
        d if d >= 10.0  => -SUPER,
        d if d >= 5.0   => -REGULAR,
        _ => 0.0,
    };

    curve.0.y = match paddle_delta.y {
        d if d <= -10.0 => -SUPER,
        d if d <= -5.0  => -REGULAR,
        d if d >= 10.0  => SUPER,
        d if d >= 5.0   => REGULAR,
        _ => 0.0,
    };
}