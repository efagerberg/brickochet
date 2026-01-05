use crate::{ball, paddle, physics, playfield};
use bevy::input::mouse;
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<mouse::MouseMotion>,
    paddle_single: Single<
        (&mut Transform, &paddle::components::PaddleSize),
        With<paddle::components::Paddle>,
    >,
    playfield: Res<playfield::resources::Playfield>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let (mut paddle_transform, paddle_size) = paddle_single.into_inner();

    let sensitivity = 0.025;
    let new_velocity = delta * sensitivity;
    let x_abs_limit = playfield.half_width - paddle_size.half_width;
    let y_abs_limit = playfield.half_height - paddle_size.half_height;

    paddle_transform.translation.x =
        (paddle_transform.translation.x + new_velocity.x).clamp(-x_abs_limit, x_abs_limit);
    paddle_transform.translation.y =
        (paddle_transform.translation.y - new_velocity.y).clamp(-y_abs_limit, y_abs_limit); // invert Y if needed
}

const Z_SPEED_INCREASE: f32 = 1.0;

pub fn paddle_ball_collision(
    ball: Single<(&Transform, &mut physics::components::Velocity), With<ball::components::Ball>>,
    paddle: Single<
        (
            &Transform,
            &paddle::components::PaddleSize,
            &mut paddle::components::PaddleMotionRecord,
        ),
        (
            With<paddle::components::Paddle>,
            Without<ball::components::Ball>,
        ),
    >,
    time: Res<Time>,
) {
    let (ball_transform, mut ball_velocity) = ball.into_inner();
    let (paddle_transform, paddle_size, mut paddle_motion_record) = paddle.into_inner();

    let p = paddle_transform.translation;
    let b = ball_transform.translation;

    // 1. Must be moving toward the paddle (+Z example)
    if ball_velocity.0.z <= 0.0 {
        return;
    }

    // 2. X overlap
    if (b.x - p.x).abs() > paddle_size.half_width + ball::components::RADIUS {
        return;
    }

    // 3. Y overlap
    if (b.y - p.y).abs() > paddle_size.half_height + ball::components::RADIUS {
        return;
    }

    // 4. Z overlap band
    let z_min = p.z - paddle_size.contact_depth - ball::components::RADIUS;
    let z_max = p.z + paddle_size.contact_depth + ball::components::RADIUS;

    if b.z < z_min || b.z > z_max {
        return;
    }

    // --- Collision confirmed ---
    // Reflect Z only
    ball_velocity.0.z = -ball_velocity.0.z - Z_SPEED_INCREASE;

    // Start motion record for curve computation
    paddle_motion_record.start_pos = Vec2::new(
        paddle_transform.translation.x,
        paddle_transform.translation.y,
    );
    paddle_motion_record.start_time = time.elapsed_secs();
    paddle_motion_record.pending = true;
}

pub fn record_paddle_motion(
    time: Res<Time>,
    paddle: Single<
        (&Transform, &mut paddle::components::PaddleMotionRecord),
        With<paddle::components::Paddle>,
    >,
) {
    let (transform, mut record) = paddle.into_inner();
    if record.pending {
        // Compute delta if 30ms have passed
        if time.elapsed_secs() - record.start_time >= 0.3 {
            let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
            record.delta = current_pos - record.start_pos;
            record.pending = false; // Done computing, ready for curve
        }
    }
}

const REGULAR_CURVE_SCALE: f32 = 0.02;
const SUPER_CURVE_SCALE: f32 = 0.05;
const SUPER_CURVE_DELTA_THRESHOLD: f32 = 10.0;
const REGULAR_CURVE_DELTA_THRESHOLD: f32 = 5.0;

pub fn apply_curve_from_motion_record(
    mut ball_curve: Single<&mut physics::components::Curve, With<ball::components::Ball>>,
    mut paddle_motion_record: Single<
        &mut paddle::components::PaddleMotionRecord,
        With<paddle::components::Paddle>,
    >,
) {
    if !paddle_motion_record.pending && paddle_motion_record.delta != Vec2::ZERO {
        // Compute curve based on motion delta over 30ms
        ball_curve.0.x = match paddle_motion_record.delta.x {
            d if d <= -SUPER_CURVE_DELTA_THRESHOLD => SUPER_CURVE_SCALE,
            d if d <= -REGULAR_CURVE_DELTA_THRESHOLD => REGULAR_CURVE_SCALE,
            d if d >= SUPER_CURVE_DELTA_THRESHOLD => -SUPER_CURVE_SCALE,
            d if d >= REGULAR_CURVE_DELTA_THRESHOLD => -REGULAR_CURVE_SCALE,
            _ => 0.0,
        };

        ball_curve.0.y = match paddle_motion_record.delta.y {
            d if d <= -SUPER_CURVE_DELTA_THRESHOLD => SUPER_CURVE_SCALE,
            d if d <= -REGULAR_CURVE_DELTA_THRESHOLD => REGULAR_CURVE_SCALE,
            d if d >= SUPER_CURVE_DELTA_THRESHOLD => -SUPER_CURVE_SCALE,
            d if d >= REGULAR_CURVE_DELTA_THRESHOLD => -REGULAR_CURVE_SCALE,
            _ => 0.0,
        };
        paddle_motion_record.delta = Vec2::ZERO;
    }
}
