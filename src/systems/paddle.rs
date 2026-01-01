use crate::components::{ball, paddle};
use bevy::input::mouse;
use bevy::prelude::*;

pub fn paddle_mouse_control(
    mut mouse_motion_message_reader: MessageReader<mouse::MouseMotion>,
    mut paddle_query: Query<&mut Transform, With<paddle::Paddle>>,
) {
    let mut delta = Vec2::ZERO;

    for ev in mouse_motion_message_reader.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.025;

    for mut transform in &mut paddle_query {
        transform.translation.x += delta.x * sensitivity;
        transform.translation.y -= delta.y * sensitivity; // invert Y if needed
    }
}

pub fn paddle_ball_collision(
    mut ball_query: Query<(&mut Transform, &mut ball::Velocity), With<ball::Ball>>,
    paddle_query: Query<(&Transform, &paddle::PaddleSize), (With<paddle::Paddle>, Without<ball::Ball>)>,
) {
    for (paddle_transform, paddle_size) in paddle_query {
        let paddle_z = paddle_transform.translation.z;
        let paddle_pos = paddle_transform.translation;

        for (mut ball_transform, mut velocity) in &mut ball_query {
            let ball_pos = ball_transform.translation;

            // Only check if ball is moving toward the paddle
            if velocity.0.z < 0.0 {
                continue;
            }

            // Has the ball crossed the paddle plane this frame?
            if ball_pos.z >= paddle_z {
                let dx = (ball_pos.x - paddle_pos.x).abs();
                let dy = (ball_pos.y - paddle_pos.y).abs();

                if dx <= paddle_size.half_width && dy <= paddle_size.half_height {
                    // Clamp ball to paddle plane
                    ball_transform.translation.z = paddle_z;

                    // Reflect Z
                    velocity.0.z *= -1.0;
                }
            }
        }
    }
}
