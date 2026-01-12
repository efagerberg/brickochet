use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Curve(pub Vec2);

#[derive(Component, Default, Clone)]
pub struct Aabb3d {
    pub half_extents: Vec3
}

// If you have a velocity and an Aabb you are a dymamic body, otherwise if you have
// an Aabb and no velocity you are static
#[derive(Component)]
pub struct Velocity(pub Vec3);