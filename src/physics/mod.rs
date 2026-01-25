use bevy::prelude::*;

pub mod components;
pub mod math;
pub mod messages;
pub mod systems;

#[cfg(test)]
mod tests;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSet {
    Forces,
    Integrate,
    DetectCollisions,
    ResolveCollisions,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<messages::CollisionMessage>()
            .configure_sets(
                FixedUpdate,
                (
                    PhysicsSet::Forces,
                    PhysicsSet::Integrate.after(PhysicsSet::Forces),
                    PhysicsSet::DetectCollisions.after(PhysicsSet::Integrate),
                    PhysicsSet::ResolveCollisions.after(PhysicsSet::DetectCollisions),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    systems::apply_curve.in_set(PhysicsSet::Forces),
                    systems::apply_velocity.in_set(PhysicsSet::Integrate),
                    systems::detect_collisions.in_set(PhysicsSet::DetectCollisions),
                    systems::resolve_sphere_aabb_collision.in_set(PhysicsSet::ResolveCollisions),
                ),
            );
    }
}
