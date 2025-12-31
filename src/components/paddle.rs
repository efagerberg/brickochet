use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct PaddleSpeed(pub f32); // units per second
