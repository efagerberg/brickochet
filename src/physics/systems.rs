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
                let normal = physics::math::sphere_aabb_contact_normal(
                    a_transform.translation,
                    a_bounds.radius,
                    b_transform.translation,
                    b_bounds.half_extents,
                );

                let contact_point = physics::math::closest_point_on_aabb(
                    a_transform.translation,
                    b_transform.translation,
                    b_bounds.half_extents,
                );

                let penetration = a_bounds.radius - contact_point.distance(a_transform.translation);

                messages.write(physics::messages::CollisionMessage {
                    a: a_entity,
                    b: b_entity,
                    normal,
                    contact_point,
                    penetration,
                });
            }
        }
    }
}

pub fn resolve_sphere_aabb_collision(
    mut messages: MessageReader<physics::messages::CollisionMessage>,
    mut sphere_query: Query<
        (&mut physics::components::Velocity, &mut Transform),
        With<physics::components::BoundingSphere>,
    >,
    cuboid_query: Query<
        Entity,
        (
            With<physics::components::BoundingCuboid>,
            Without<physics::components::BoundingSphere>,
        ),
    >,
) {
    // Collect all collisions per sphere
    let mut collisions_per_sphere: std::collections::HashMap<
        Entity,
        Vec<&physics::messages::CollisionMessage>,
    > = std::collections::HashMap::new();
    for message in messages.read() {
        collisions_per_sphere
            .entry(message.a)
            .or_default()
            .push(message);
    }

    for (sphere_entity, collisions) in collisions_per_sphere {
        if let Ok((mut velocity, mut transform)) = sphere_query.get_mut(sphere_entity) {
            let mut total_normal = Vec3::ZERO;
            let mut max_penetration: f32 = 0.0;

            // Only consider collisions with valid cuboids
            for message in collisions {
                if cuboid_query.get(message.b).is_ok() {
                    total_normal += message.normal;
                    max_penetration = max_penetration.max(message.penetration);
                }
            }

            if total_normal != Vec3::ZERO {
                let normal = total_normal.normalize();

                // Move the sphere out of the cuboid
                transform.translation += normal * max_penetration;

                // Reflect velocity once
                velocity.0 = velocity.0.reflect(normal);
            }
        }
    }
}
