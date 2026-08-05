use bevy::prelude::*;

use crate::gameplay::paddle;
use crate::gameplay::player;
use crate::gameplay::playfield;
use crate::physics;
use crate::rendering;

pub fn ball_to_paddle_distance_glow(
    ball_query: Single<(Entity, &Transform), With<physics::components::SphereCollider>>,
    paddle_query: Single<
        &Transform,
        (
            With<paddle::components::Paddle>,
            With<player::components::Player>,
        ),
    >,
    goal_query: Query<(&playfield::components::Goal, &Transform)>,
    playfield: Res<playfield::resources::Playfield>,
    mut messages: MessageWriter<rendering::messages::MaterialColorsChangedMessage>,
) {
    let (ball_entity, ball_transform) = ball_query.into_inner();
    let paddle_transform = paddle_query.into_inner();

    let enemy_goal = goal_query
        .iter()
        .find(|(goal, _)| **goal == playfield::components::Goal::Enemy);

    if let Some((_, goal_transform)) = enemy_goal {
        let ball_z = ball_transform.translation.z;
        let distance = (paddle_transform.translation.z - ball_z).abs();
        let max_distance = (paddle_transform.translation.z - goal_transform.translation.z).abs();
        let t = (distance / max_distance).clamp(0.0, 1.0); // 0 if far, 1 if very closet);
        let new_color = LinearRgba::mix(
            &playfield.ball_distance_near_color,
            &playfield.ball_distance_far_color,
            t,
        );
        messages.write(rendering::messages::MaterialColorsChangedMessage {
            entity: ball_entity,
            emissive: Some(new_color),
            base_color: None,
        });
    }
}
