use crate::physics;
use bevy::prelude::*;

pub fn apply_velocity(
    time: Res<Time>,
    query: Query<(&mut Transform, &physics::components::Velocity)>,
) {
    for (mut transform, velocity) in query {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

pub fn apply_curve(
    query: Query<(
        &mut physics::components::Velocity,
        &physics::components::Curve,
    )>,
) {
    for (mut velocity, curve) in query {
        velocity.0.x += curve.0.x;
        velocity.0.y += curve.0.y;
    }
}
