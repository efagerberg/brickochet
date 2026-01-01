use crate::components::ball;
use crate::resources::playfield;
use bevy::prelude::*;

pub fn move_ball(
    time: Res<Time>,
    ball: Single<(&mut Transform, &ball::Velocity), With<ball::Ball>>,
) {
    let (mut ball_transform, ball_velocity) = ball.into_inner();
    ball_transform.translation += ball_velocity.0 * time.delta_secs();
}

pub fn reflect_ball(
    playfield: Res<playfield::Playfield>,
    ball: Single<(&mut Transform, &mut ball::Velocity), With<ball::Ball>>,
) {
    let (mut ball_transform, mut ball_velocity) = ball.into_inner();
    let pos = ball_transform.translation;

    // X walls
    if pos.x > playfield.half_width {
        ball_transform.translation.x = playfield.half_width;
        ball_velocity.0.x *= -1.0;
        return;
    }
    if pos.x < -playfield.half_width {
        ball_transform.translation.x = -playfield.half_width;
        ball_velocity.0.x *= -1.0;
        return;
    }

    if pos.y > playfield.half_height {
        ball_transform.translation.y = playfield.half_height;
        ball_velocity.0.y *= -1.0;
        return;
    }
    if pos.y < -playfield.half_height {
        ball_transform.translation.y = -playfield.half_height;
        ball_velocity.0.y *= -1.0;
        return;
    }

    if pos.z < -playfield.half_depth {
        ball_transform.translation.z = -playfield.half_depth;
        ball_velocity.0.z *= -1.0;
        return;
    }
}
