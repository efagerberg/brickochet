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
    DetectCollisions,
    ResolveCollisions,
    Integrate,
}

pub fn plugin(app: &mut App) {
    app.add_message::<messages::CollisionMessage>()
        .configure_sets(
            FixedUpdate,
            (
                PhysicsSet::ComputeForces,
                PhysicsSet::DetectCollisions.after(PhysicsSet::ComputeForces),
                PhysicsSet::ResolveCollisions.after(PhysicsSet::DetectCollisions),
                PhysicsSet::Integrate.after(PhysicsSet::ResolveCollisions),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                systems::add_curve_velocity.in_set(PhysicsSet::ComputeForces),
                systems::detect_collisions.in_set(PhysicsSet::DetectCollisions),
                systems::resolve_sphere_aabb_collision.in_set(PhysicsSet::ResolveCollisions),
                (systems::apply_velocity, systems::apply_curve_spin).in_set(PhysicsSet::Integrate),
            ),
        );
}
