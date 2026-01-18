use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Curve(pub Vec2);

#[derive(Component, Default, Clone)]
pub struct BoundingCuboid {
    pub half_extents: Vec3,
}

#[derive(Component, Default, Clone)]
pub struct BoundingSphere {
    pub radius: f32,
}

// If you have a velocity and an Bounding Cuboid you are a dynamic body, otherwise if you have
// an Bounding Cuboid and no velocity you are static
#[derive(Component)]
pub struct Velocity(pub Vec3);
