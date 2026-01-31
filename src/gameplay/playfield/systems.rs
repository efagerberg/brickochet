use bevy::prelude::*;

use crate::gameplay::{ball, playfield};
use crate::physics;
use crate::rendering;

pub fn highlight_depth_lines(
    ball_query: Single<(&Transform, &physics::components::BoundingSphere)>,
    lines: Query<(Entity, &Transform), With<playfield::components::DepthLines>>,
    playfield: Res<playfield::resources::Playfield>,
    mut messages: MessageWriter<rendering::messages::MaterialColorsChangedMessage>,
) {
    let (ball_transform, sphere) = ball_query.into_inner();

    let ball_z = ball_transform.translation.z;
    // 2 ball diameters distance away, increase for smoothing animation, decrease
    // to make animation more choppy
    let max_distance = 2.0 * sphere.radius * 2.0;
    let base_color = &playfield.wall_line_default_color;
    let highlight_color = &playfield.wall_line_highlight_color;

    for (entity, line_transform) in lines {
        let distance = (line_transform.translation.z - ball_z).abs();
        let t = (max_distance - distance).clamp(0.0, 1.0); // 0 if far, 1 if very closet);
        let new_color = LinearRgba::mix(base_color, highlight_color, t);

        messages.write(rendering::messages::MaterialColorsChangedMessage {
            entity,
            emissive: Some(new_color),
            base_color: None,
        });
    }
}

pub fn handle_wall_collision(
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
                ball_transform.translation = Vec3::default();
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
