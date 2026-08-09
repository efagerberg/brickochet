use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Curve(pub Vec2);

/// A collider for a three-dimensional shape with six rectangular faces and all right
/// angles. Half extents is used as a convenience for collision detection as it avoids
/// a division. Ex center ± half_extents, not center ± size/2
#[derive(Component, Default, Clone, Reflect)]
pub struct CuboidCollider {
    pub half_extents: Vec3,
}

#[derive(Component, Default, Clone, Reflect)]
pub struct SphereCollider {
    pub radius: f32,
}

/// A collider for a bounded rectangular plane shape with one face. Half extents is used
/// as a convenience for collision detection as it avoids a division.
/// Ex center ± half_extents, not center ± size/2
#[derive(Component, Clone, Reflect)]
pub struct PlaneCollider {
    pub half_extents: Vec2,
    pub normal: Vec3,
}

#[derive(Component, Reflect)]
pub struct Velocity(pub Vec3);

/// A dynamic body: moved and reflected by the physics simulation itself
/// (integrated by `apply_velocity`, has its Velocity reflected by
/// `resolve_sphere_aabb_collision` on collision). Your ball.
#[derive(Component)]
pub struct DynamicBody;

/// A kinematic body: has a Velocity that's integrated by `apply_velocity`
/// like a dynamic body, but that Velocity is set by something outside the
/// physics module (player input, a scripted path) rather than by forces
/// or collision response. Dynamic bodies detect and react to it; it never
/// reacts to them. Your paddle.
#[derive(Component)]
pub struct KinematicBody;

/// A static body: never moves, has no Velocity at all. Detected against,
/// never integrated. Your walls.
#[derive(Component)]
pub struct StaticBody;
