use crate::components::{ball, physics};
use crate::resources::playfield;
use bevy::prelude::*;

pub fn reflect_ball(
    playfield: Res<playfield::Playfield>,
    ball: Single<(&mut Transform, &mut physics::Velocity), With<ball::Ball>>,
) {
    let (mut ball_transform, mut ball_velocity) = ball.into_inner();
    let pos = ball_transform.translation;

    // X walls
    if pos.x > playfield.half_width {
        ball_transform.translation.x = playfield.half_width;
        ball_velocity.0.x *= -1.0;
    }
    if pos.x < -playfield.half_width {
        ball_transform.translation.x = -playfield.half_width;
        ball_velocity.0.x *= -1.0;
    }

    if pos.y > playfield.half_height {
        ball_transform.translation.y = playfield.half_height;
        ball_velocity.0.y *= -1.0;
    }
    if pos.y < -playfield.half_height {
        ball_transform.translation.y = -playfield.half_height;
        ball_velocity.0.y *= -1.0;
    }

    if pos.z < -playfield.half_depth {
        ball_transform.translation.z = -playfield.half_depth;
        ball_velocity.0.z *= -1.0;
    }
}
