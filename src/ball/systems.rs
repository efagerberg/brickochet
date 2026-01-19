use crate::{ball, physics, playfield};
use bevy::prelude::*;

pub fn wall_collision_handler(
    playfield: Res<playfield::resources::Playfield>,
    ball: Single<(
        &physics::components::BoundingSphere,
        &ball::components::BallModifiers,
        &mut Transform,
        &mut physics::components::Velocity,
        &mut physics::components::Curve,
    )>,
) {
    let (sphere, ball_modifiers, mut ball_transform, mut ball_velocity, mut curve) =
        ball.into_inner();

    if ball_transform.translation.z + sphere.radius > playfield.bounds.half_extents.z {
        ball_transform.translation = Vec3::default();
        ball_velocity.0 = ball_modifiers.base_velocity;
        curve.0 = Vec2::ZERO;
    } else if ball_transform.translation.z - sphere.radius < -playfield.bounds.half_extents.z {
        // For now clear curve on ball wall. In Curveball the ball spin is set when
        // the enemy AI hits the ball, this tries to mimic that feel. Probably when
        // bricks are added, they will do the same.
        curve.0 = Vec2::ZERO;
    }
}
