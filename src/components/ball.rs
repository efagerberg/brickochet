use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub const BALL_RADIUS: f32 = 1.0;
