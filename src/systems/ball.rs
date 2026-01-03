use crate::components::{ball, physics};
use crate::resources::playfield;
use bevy::prelude::*;

pub fn reflect_ball(
    playfield: Res<playfield::Playfield>,
    ball: Single<(&mut Transform, &mut physics::Velocity, &mut physics::Curve), With<ball::Ball>>,
) {
    let (mut ball_transform, mut ball_velocity, mut curve) = ball.into_inner();

    if ball_transform.translation.x + ball::RADIUS > playfield.half_width {
        ball_transform.translation.x = playfield.half_width - ball::RADIUS;
        ball_velocity.0.x *= -1.0;
    }
    if ball_transform.translation.x - ball::RADIUS < -playfield.half_width {
        ball_transform.translation.x = -playfield.half_width + ball::RADIUS;
        ball_velocity.0.x *= -1.0;
    }

    if ball_transform.translation.y + ball::RADIUS > playfield.half_height {
        ball_transform.translation.y = playfield.half_height - ball::RADIUS;
        ball_velocity.0.y *= -1.0;
    }
    if ball_transform.translation.y - ball::RADIUS < -playfield.half_height {
        ball_transform.translation.y = -playfield.half_height + ball::RADIUS;
        ball_velocity.0.y *= -1.0;
    }

    if ball_transform.translation.z + ball::RADIUS > playfield.half_depth {
        ball_transform.translation = Vec3::default();
        ball_velocity.0 = Vec3::new(0.0, 0.0, 15.0);
        curve.0 = Vec2::ZERO;
    }
    if ball_transform.translation.z - ball::RADIUS < -playfield.half_depth {
        ball_transform.translation.z = -playfield.half_depth + ball::RADIUS;
        ball_velocity.0.z *= -1.0;
    }
}
