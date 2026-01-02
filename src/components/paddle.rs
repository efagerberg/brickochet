use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct PaddleSize {
    pub half_width: f32,
    pub half_height: f32,
    pub contact_depth: f32,
}

#[derive(Component, Default)]
pub struct PaddleMotionRecord {
    pub start_pos: Vec2, // Position at collision
    pub start_time: f32, // Time at collision
    pub delta: Vec2,     // Computed delta over window
    pub pending: bool,   // Is a curve calculation pending?
}
