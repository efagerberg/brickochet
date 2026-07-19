use bevy::prelude::*;

use crate::gameplay::{ball, brick, playfield};
use crate::physics;

pub fn track_ball_with_depth_line(
    ball_query: Single<
        &Transform,
        (
            With<physics::components::BoundingSphere>,
            Without<playfield::components::DepthLine>,
        ),
    >,
    lines: Query<&mut Transform, With<playfield::components::DepthLine>>,
    playfield: Res<playfield::resources::Playfield>,
) {
    let ball_transform = ball_query.into_inner();

    let tracking_z = ball_transform
        .translation
        .z
        .min(playfield.half_size.z)
        .max(-playfield.half_size.z);
    for mut line_transform in lines {
        line_transform.translation.z = tracking_z;
    }
}

pub fn handle_wall_collision(
    mut commands: Commands,
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        (
            &ball::components::BallModifiers,
            &mut Transform,
            &mut physics::components::Velocity,
            &mut physics::components::Curve,
        ),
        With<physics::components::BoundingSphere>,
    >,
    goal_query: Query<&playfield::components::Goal, With<physics::components::BoundingCuboid>>,
) {
    for message in messages.read() {
        let (Ok((ball_modifiers, mut ball_transform, mut ball_velocity, mut curve)), Ok(goal)) =
            (sphere_query.get_mut(message.a), goal_query.get(message.b))
        else {
            continue;
        };

        match goal {
            playfield::components::Goal::Player => {
                commands.entity(message.a).remove::<(
                    brick::components::RicochetCurveEffect,
                    brick::components::RicochetSizeEffect,
                    brick::components::RicochetSpeedEffect,
                )>();
                ball_transform.translation = Vec3::default();
                ball_transform.scale = Vec3::ONE;
                ball_transform.rotation = Quat::IDENTITY;
                ball_velocity.0 = ball_modifiers.base_velocity;
                curve.0 = Vec2::ZERO;
            }
            playfield::components::Goal::Enemy => {
                // For now clear curve on ball wall. In Curveball the ball spin is set when
                // the enemy AI hits the ball, this tries to mimic that feel. Probably when
                // bricks are added, they will do the same.
                curve.0 = Vec2::ZERO;
            }
        }
    }
}
