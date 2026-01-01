use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct PaddleSize {
    pub half_width: f32,
    pub half_height: f32,
    pub contact_depth: f32,
}
