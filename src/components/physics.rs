use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct Curve(pub Vec2);
