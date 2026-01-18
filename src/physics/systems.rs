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

pub fn detect_collisions(
    spheres: Query<(Entity, &Transform, &physics::components::BoundingSphere)>,
    cuboids: Query<(Entity, &Transform, &physics::components::BoundingCuboid)>,
    mut messages: MessageWriter<physics::messages::CollisionMessage>,
) {
    for (a_entity, a_transform, a_bounds) in spheres.iter() {
        for (b_entity, b_transform, b_bounds) in cuboids.iter() {
            if physics::math::sphere_aabb_intersects(
                a_transform.translation,
                a_bounds.radius,
                b_transform.translation,
                b_bounds.half_extents,
            ) {
                messages.write(physics::messages::CollisionMessage {
                    a: a_entity,
                    b: b_entity,
                });
            }
        }
    }
}

pub fn reflect_sphere(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        &mut physics::components::Velocity,
        With<physics::components::BoundingSphere>,
    >,
) {
    for message in messages.read() {
        if let Ok(mut sphere_velocity) = sphere_query.get_mut(message.a) {
            // Reflect Z only for simplified but more consistent physics
            let reflect_scale = -sphere_velocity.0.z.signum();
            let z_magnitude = sphere_velocity.0.z.abs();
            sphere_velocity.0.z = reflect_scale * z_magnitude;
        }
    }
}
