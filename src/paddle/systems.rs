use crate::{ball, paddle, physics, playfield};
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<bevy::input::mouse::MouseMotion>,
    paddle_single: Single<
        (&mut Transform, &physics::components::BoundingCuboid),
        With<paddle::components::Paddle>,
    >,
    goal_query: Query<(
        &playfield::components::Goal,
        &physics::components::BoundingCuboid,
    )>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let (mut paddle_transform, paddle_bounds) = paddle_single.into_inner();

    let enemy_goal = goal_query
        .iter()
        .find(|(goal, _)| **goal == playfield::components::Goal::Enemy);

    if let Some((_, bounds)) = enemy_goal {
        let sensitivity = 0.025;
        let new_velocity = delta * sensitivity;
        let x_abs_limit = bounds.half_extents.x - paddle_bounds.half_extents.x;
        let y_abs_limit = bounds.half_extents.y - paddle_bounds.half_extents.y;

        paddle_transform.translation.x =
            (paddle_transform.translation.x + new_velocity.x).clamp(-x_abs_limit, x_abs_limit);
        paddle_transform.translation.y =
            (paddle_transform.translation.y - new_velocity.y).clamp(-y_abs_limit, y_abs_limit); // invert Y if needed
    }
}

pub fn apply_paddle_impact_modifiers(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        &mut physics::components::Velocity,
        With<physics::components::BoundingSphere>,
    >,
    mut paddle_query: Query<
        &paddle::components::PaddleImpactModifiers,
        With<paddle::components::Paddle>,
    >,
) {
    for message in messages.read() {
        if let (Ok(mut sphere_velocity), Ok(paddle_modifiers)) = (
            sphere_query.get_mut(message.a),
            paddle_query.get_mut(message.b),
        ) {
            let z_direction = sphere_velocity.0.z.signum();
            sphere_velocity.0.z += z_direction * paddle_modifiers.z_speed_delta;
        }
    }
}

pub fn initialize_paddle_motion(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut paddle_query: Query<
        (&Transform, &mut paddle::components::PaddleMotionRecord),
        (With<paddle::components::Paddle>,),
    >,
    time: Res<Time>,
) {
    for message in messages.read() {
        if let Ok((paddle_transform, mut paddle_motion_record)) = paddle_query.get_mut(message.b) {
            // Start motion record for curve computation
            paddle_motion_record.start_pos = Vec2::new(
                paddle_transform.translation.x,
                paddle_transform.translation.y,
            );
            paddle_motion_record.start_time = time.elapsed_secs();
            paddle_motion_record.pending = true;
        }
    }
}

pub fn finalize_paddle_motion(
    time: Res<Time>,
    paddle_query: Single<
        (&Transform, &mut paddle::components::PaddleMotionRecord),
        With<paddle::components::Paddle>,
    >,
    window_query: Single<&Window>,
) {
    let window = window_query.into_inner();
    let width = window.width();
    let height = window.height();
    let (transform, mut record) = paddle_query.into_inner();
    // Only update if 200ms has elapsed from start of collision
    if record.pending && time.elapsed_secs() - record.start_time >= 0.2 {
        let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let raw_delta = current_pos - record.start_pos;
        record.delta = Vec2::new(raw_delta.x / width, raw_delta.y / height);
        record.pending = false; // Done computing, ready for curve
    }
}

pub fn apply_curve_from_motion_record(
    mut ball_curve: Single<&mut physics::components::Curve, With<ball::components::BallModifiers>>,
    paddle: Single<
        (
            &mut paddle::components::PaddleMotionRecord,
            &paddle::components::PaddleImpactModifiers,
        ),
        With<paddle::components::Paddle>,
    >,
) {
    let (mut motion_record, modifiers) = paddle.into_inner();
    if motion_record.pending || motion_record.delta == Vec2::ZERO {
        return;
    }

    // Compute curve based on motion delta over 30ms
    ball_curve.0.x = match motion_record.delta.x {
        d if d <= -modifiers.super_curve_position_delta_threshold => modifiers.super_curve_scale,
        d if d <= -modifiers.normal_curve_position_delta_threshold => modifiers.normal_curve_scale,
        d if d >= modifiers.super_curve_position_delta_threshold => -modifiers.super_curve_scale,
        d if d >= modifiers.normal_curve_position_delta_threshold => -modifiers.normal_curve_scale,
        _ => 0.0,
    };

    ball_curve.0.y = match motion_record.delta.y {
        d if d <= -modifiers.super_curve_position_delta_threshold => modifiers.super_curve_scale,
        d if d <= -modifiers.normal_curve_position_delta_threshold => modifiers.normal_curve_scale,
        d if d >= modifiers.super_curve_position_delta_threshold => -modifiers.super_curve_scale,
        d if d >= modifiers.normal_curve_position_delta_threshold => -modifiers.normal_curve_scale,
        _ => 0.0,
    };
    motion_record.delta = Vec2::ZERO;
}
