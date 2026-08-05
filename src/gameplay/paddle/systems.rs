use bevy::prelude::*;

use crate::gameplay::{ball, paddle, playfield};
use crate::physics;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<bevy::input::mouse::MouseMotion>,
    time: Res<Time>,
    paddle_single: Single<
        (
            &Transform,
            &mut physics::components::Velocity,
            &physics::components::CuboidCollider,
        ),
        With<paddle::components::Paddle>,
    >,
    goal_query: Query<(
        &playfield::components::Goal,
        &physics::components::CuboidCollider,
    )>,
    cursor_options: Single<&bevy::window::CursorOptions>,
) {
    let (paddle_transform, mut paddle_velocity, paddle_collider) = paddle_single.into_inner();

    if cursor_options.visible {
        // Cursor visible (paused/menu, etc): stop the paddle rather than
        // leaving last frame's velocity for apply_velocity to keep
        // integrating while this system isn't producing new input.
        paddle_velocity.0 = Vec3::ZERO;
        return;
    }

    let mut delta = Vec2::ZERO;
    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        // No mouse movement this frame: stop immediately, don't coast on
        // a stale nonzero velocity from the last frame that did move.
        paddle_velocity.0 = Vec3::ZERO;
        return;
    }

    let delta_secs = time.delta_secs();

    let sensitivity = 0.025;
    let movement = delta * sensitivity; // desired displacement this frame, same as before

    // Clamp against the goal collider using the *current* position, exactly
    // as the direct-mutation version did, then back out whatever velocity
    // reproduces this same (possibly clamped) displacement once
    // apply_velocity integrates it as `velocity * delta_secs`.
    let mut target_x = paddle_transform.translation.x + movement.x;
    // TODO: Inverse this based on player preferences
    let mut target_y = paddle_transform.translation.y - movement.y;

    let enemy_goal = goal_query
        .iter()
        .find(|(goal, _)| **goal == playfield::components::Goal::Enemy);

    if let Some((_, enemy_goal_collider)) = enemy_goal {
        let x_abs_limit = enemy_goal_collider.half_extents.x - paddle_collider.half_extents.x;
        let y_abs_limit = enemy_goal_collider.half_extents.y - paddle_collider.half_extents.y;
        target_x = target_x.clamp(-x_abs_limit, x_abs_limit);
        target_y = target_y.clamp(-y_abs_limit, y_abs_limit);
    };

    let clamped_displacement = Vec3::new(
        target_x - paddle_transform.translation.x,
        target_y - paddle_transform.translation.y,
        0.0,
    );

    paddle_velocity.0 = clamped_displacement / delta_secs;
}

pub fn apply_paddle_impact_modifiers(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        &mut physics::components::Velocity,
        With<physics::components::SphereCollider>,
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
