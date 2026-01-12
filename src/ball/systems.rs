use crate::{ball, physics, playfield};
use bevy::prelude::*;

pub fn reflect_ball(
    playfield: Res<playfield::resources::Playfield>,
    ball: Single<(
        &ball::components::BallModifiers,
        &mut Transform,
        &mut physics::components::Velocity,
        &mut physics::components::Curve,
    )>,
) {
    let (ball_modifiers, mut ball_transform, mut ball_velocity, mut curve) = ball.into_inner();

    if ball_transform.translation.x + ball_modifiers.radius > playfield.aabb.half_extents.x {
        ball_transform.translation.x = playfield.aabb.half_extents.x - ball_modifiers.radius;
        ball_velocity.0.x *= -1.0;
    } else if ball_transform.translation.x - ball_modifiers.radius < -playfield.aabb.half_extents.x {
        ball_transform.translation.x = -playfield.aabb.half_extents.x + ball_modifiers.radius;
        ball_velocity.0.x *= -1.0;
    }

    if ball_transform.translation.y + ball_modifiers.radius > playfield.aabb.half_extents.y {
        ball_transform.translation.y = playfield.aabb.half_extents.y - ball_modifiers.radius;
        ball_velocity.0.y *= -1.0;
    } else if ball_transform.translation.y - ball_modifiers.radius < -playfield.aabb.half_extents.y {
        ball_transform.translation.y = -playfield.aabb.half_extents.y + ball_modifiers.radius;
        ball_velocity.0.y *= -1.0;
    }

    if ball_transform.translation.z + ball_modifiers.radius > playfield.aabb.half_extents.z {
        ball_transform.translation = Vec3::default();
        ball_velocity.0 = ball_modifiers.base_velocity;
        curve.0 = Vec2::ZERO;
    } else if ball_transform.translation.z - ball_modifiers.radius < -playfield.aabb.half_extents.z {
        ball_transform.translation.z = -playfield.aabb.half_extents.z + ball_modifiers.radius;
        ball_velocity.0.z *= -1.0;
        // For now clear curve on ball wall. In Curveball the ball spin is set when
        // the enemy AI hits the ball, this tries to mimic that feel. Probably when
        // bricks are added, they will do the same.
        curve.0 = Vec2::ZERO;
    }
}
