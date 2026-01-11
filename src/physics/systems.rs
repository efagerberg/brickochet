use crate::physics;
use bevy::prelude::*;

pub fn apply_velocity(
    time: Res<Time>,
    query: Query<(&mut Transform, &physics::components::Velocity)>,
) {
    let delta_secs = time.delta_secs();
    for (mut transform, velocity) in query {
        transform.translation += velocity.0 * delta_secs;
    }
}

pub fn apply_curve(
    time: Res<Time>,
    query: Query<(
        &mut physics::components::Velocity,
        &physics::components::Curve,
    )>,
) {
    let delta_secs = time.delta_secs();
    for (mut velocity, curve) in query {
        velocity.0.x += curve.0.x * delta_secs;
        velocity.0.y += curve.0.y * delta_secs;
    }
}
