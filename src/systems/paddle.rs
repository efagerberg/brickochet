use crate::components::{ball, paddle, physics};
use bevy::input::mouse;
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<mouse::MouseMotion>,
    mut paddle_transform: Single<&mut Transform, With<paddle::Paddle>>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.025;
    let new_velocity = delta * sensitivity;

    paddle_transform.translation.x += new_velocity.x;
    paddle_transform.translation.y -= new_velocity.y; // invert Y if needed
}

pub fn paddle_ball_collision(
    ball: Single<(&mut Transform, &mut physics::Velocity), With<ball::Ball>>,
    paddle: Single<
        (
            &Transform,
            &paddle::PaddleSize,
            &mut paddle::PaddleMotionRecord,
        ),
        (With<paddle::Paddle>, Without<ball::Ball>),
    >,
    time: Res<Time>,
) {
    let (mut ball_transform, mut ball_velocity) = ball.into_inner();
    let (paddle_transform, paddle_size, mut paddle_motion_record) = paddle.into_inner();

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
    paddle: Single<(&Transform, &mut paddle::PaddleMotionRecord), With<paddle::Paddle>>,
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

pub fn apply_curve_from_motion_record(
    mut ball_curve: Single<&mut physics::Curve, With<ball::Ball>>,
    paddle_motion_record: Single<&paddle::PaddleMotionRecord, With<paddle::Paddle>>,
) {
    if !paddle_motion_record.pending && paddle_motion_record.delta != Vec2::ZERO {
        const REGULAR: f32 = 0.002;
        const SUPER: f32 = 0.0005;

        ball_curve.0.x = match paddle_motion_record.delta.x {
            d if d <= -10.0 => SUPER,
            d if d <= -5.0  => REGULAR,
            d if d >= 10.0  => -SUPER,
            d if d >= 5.0   => -REGULAR,
            _ => 0.0,
        };

        ball_curve.0.y = match paddle_motion_record.delta.y {
            d if d <= -10.0 => -SUPER,
            d if d <= -5.0  => -REGULAR,
            d if d >= 10.0  => SUPER,
            d if d >= 5.0   => REGULAR,
            _ => 0.0,
        };
    }
}

