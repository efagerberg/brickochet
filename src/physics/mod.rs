use bevy::prelude::*;

pub mod components;
pub mod math;
pub mod messages;
pub mod systems;

#[cfg(test)]
mod tests;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSet {
    ComputeForces,
    ApplyForces,
    DetectCollisions,
    ResolveCollisions,
}

pub fn plugin(app: &mut App) {
    app.add_message::<messages::CollisionMessage>()
        .configure_sets(
            FixedUpdate,
            (
                PhysicsSet::ComputeForces,
                PhysicsSet::ApplyForces.after(PhysicsSet::ComputeForces),
                PhysicsSet::DetectCollisions.after(PhysicsSet::ApplyForces),
                PhysicsSet::ResolveCollisions.after(PhysicsSet::DetectCollisions),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                systems::add_curve_velocity.in_set(PhysicsSet::ComputeForces),
                (systems::apply_velocity, systems::apply_curve_spin)
                    .in_set(PhysicsSet::ApplyForces),
                systems::detect_collisions.in_set(PhysicsSet::DetectCollisions),
                systems::resolve_sphere_aabb_collision.in_set(PhysicsSet::ResolveCollisions),
            ),
        );
}
