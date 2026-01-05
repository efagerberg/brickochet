use crate::{ball, physics, playfield};
use bevy::prelude::*;

pub fn reflect_ball(
    playfield: Res<playfield::resources::Playfield>,
    ball: Single<
        (
            &mut Transform,
            &mut physics::components::Velocity,
            &mut physics::components::Curve,
        ),
        With<ball::components::Ball>,
    >,
) {
    let (mut ball_transform, mut ball_velocity, mut curve) = ball.into_inner();

    if ball_transform.translation.x + ball::components::RADIUS > playfield.half_width {
        ball_transform.translation.x = playfield.half_width - ball::components::RADIUS;
        ball_velocity.0.x *= -1.0;
    }
    else if ball_transform.translation.x - ball::components::RADIUS < -playfield.half_width {
        ball_transform.translation.x = -playfield.half_width + ball::components::RADIUS;
        ball_velocity.0.x *= -1.0;
    }

    if ball_transform.translation.y + ball::components::RADIUS > playfield.half_height {
        ball_transform.translation.y = playfield.half_height - ball::components::RADIUS;
        ball_velocity.0.y *= -1.0;
    }
    else if ball_transform.translation.y - ball::components::RADIUS < -playfield.half_height {
        ball_transform.translation.y = -playfield.half_height + ball::components::RADIUS;
        ball_velocity.0.y *= -1.0;
    }

    if ball_transform.translation.z + ball::components::RADIUS > playfield.half_depth {
        ball_transform.translation = Vec3::default();
        ball_velocity.0 = ball::components::DEFAULT_VELOCITY;
        curve.0 = Vec2::ZERO;
    }
    else if ball_transform.translation.z - ball::components::RADIUS < -playfield.half_depth {
        ball_transform.translation.z = -playfield.half_depth + ball::components::RADIUS;
        ball_velocity.0.z *= -1.0;
        // For now clear curve on ball wall. In Curveball the ball spin is set when
        // the enemy AI hits the ball, this tries to mimic that feel. Probably when
        // bricks are added, they will do the same.
        curve.0 = Vec2::ZERO;
    }
}
