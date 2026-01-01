use crate::components::ball;
use crate::resources::playfield;
use bevy::prelude::*;

pub fn move_ball(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ball::Velocity), With<ball::Ball>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

pub fn reflect_ball(
    playfield: Res<playfield::Playfield>,
    mut query: Query<(&mut Transform, &mut ball::Velocity), With<ball::Ball>>,
) {
    for (mut transform, mut velocity) in &mut query {
        let pos = transform.translation;

        // X walls
        if pos.x > playfield.half_width {
            transform.translation.x = playfield.half_width;
            velocity.0.x *= -1.0;
            continue;
        }
        if pos.x < -playfield.half_width {
            transform.translation.x = -playfield.half_width;
            velocity.0.x *= -1.0;
            continue;
        }

        if pos.y > playfield.half_height {
            transform.translation.y = playfield.half_height;
            velocity.0.y *= -1.0;
            continue;
        }
        if pos.y < -playfield.half_height {
            transform.translation.y = -playfield.half_height;
            velocity.0.y *= -1.0;
            continue;
        }

        if pos.z < -playfield.half_depth {
            transform.translation.z = -playfield.half_depth;
            velocity.0.z *= -1.0;
            continue;
        }
    }
}
