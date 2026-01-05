use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

pub const RADIUS: f32 = 0.75;
pub const DEFAULT_VELOCITY: Vec3 = Vec3::new(0.0,0.0,25.0);
