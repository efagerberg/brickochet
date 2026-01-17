use crate::{ball, paddle, physics, playfield};
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<bevy::input::mouse::MouseMotion>,
    paddle_single: Single<
        (&mut Transform, &physics::components::BoundingCuboid),
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

    let (mut paddle_transform, bounds) = paddle_single.into_inner();

    let sensitivity = 0.025;
    let new_velocity = delta * sensitivity;
    let x_abs_limit = playfield.bounds.half_extents.x - bounds.half_extents.x;
    let y_abs_limit = playfield.bounds.half_extents.y - bounds.half_extents.y;

    paddle_transform.translation.x =
        (paddle_transform.translation.x + new_velocity.x).clamp(-x_abs_limit, x_abs_limit);
    paddle_transform.translation.y =
        (paddle_transform.translation.y - new_velocity.y).clamp(-y_abs_limit, y_abs_limit); // invert Y if needed
}

pub fn paddle_sphere_collision(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        &mut physics::components::Velocity,
        With<physics::components::BoundingSphere>,
    >,
    mut paddle_query: Query<
        (
            &Transform,
            &paddle::components::PaddleImpactModifiers,
            &mut paddle::components::PaddleMotionRecord,
        ),
        (
            With<paddle::components::Paddle>,
            Without<ball::components::BallModifiers>,
        ),
    >,
    time: Res<Time>,
) {
    for message in messages.read() {
        if let (
            Ok(mut sphere_velocity),
            Ok((paddle_transform, paddle_modifiers, mut paddle_motion_record)),
        ) = (
            sphere_query.get_mut(message.a),
            paddle_query.get_mut(message.b),
        ) {
            // Reflect Z only for simplified but more consistent physics
            sphere_velocity.0.z = -sphere_velocity.0.z - paddle_modifiers.contact_z_speed_increase;

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

pub fn record_paddle_motion(
    time: Res<Time>,
    paddle: Single<
        (&Transform, &mut paddle::components::PaddleMotionRecord),
        With<paddle::components::Paddle>,
    >,
) {
    let (transform, mut record) = paddle.into_inner();
    if record.pending {
        // Compute delta if 200ms have passed
        if time.elapsed_secs() - record.start_time >= 0.2 {
            let current_pos = Vec2::new(transform.translation.x, transform.translation.y);
            record.delta = current_pos - record.start_pos;
            record.pending = false; // Done computing, ready for curve
        }
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
    if !motion_record.pending && motion_record.delta != Vec2::ZERO {
        // Compute curve based on motion delta over 30ms
        ball_curve.0.x = match motion_record.delta.x {
            d if d <= -modifiers.super_curve_position_delta_threshold => {
                modifiers.super_curve_scale
            }
            d if d <= -modifiers.normal_curve_position_delta_threshold => {
                modifiers.normal_curve_scale
            }
            d if d >= modifiers.super_curve_position_delta_threshold => {
                -modifiers.super_curve_scale
            }
            d if d >= modifiers.normal_curve_position_delta_threshold => {
                -modifiers.normal_curve_scale
            }
            _ => 0.0,
        };

        ball_curve.0.y = match motion_record.delta.y {
            d if d <= -modifiers.super_curve_position_delta_threshold => {
                modifiers.super_curve_scale
            }
            d if d <= -modifiers.normal_curve_position_delta_threshold => {
                modifiers.normal_curve_scale
            }
            d if d >= modifiers.super_curve_position_delta_threshold => {
                -modifiers.super_curve_scale
            }
            d if d >= modifiers.normal_curve_position_delta_threshold => {
                -modifiers.normal_curve_scale
            }
            _ => 0.0,
        };
        motion_record.delta = Vec2::ZERO;
    }
}
