use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Default)]
pub struct PaddleMotionRecord {
    pub start_pos: Vec2, // Position at collision
    pub start_time: f32, // Time at collision
    pub delta: Vec2,     // Computed delta over window
    pub pending: bool,   // Is a curve calculation pending?
}

#[derive(Component, Default)]
pub struct PaddleImpactModifiers {
    pub normal_curve_scale: f32,
    pub super_curve_scale: f32,
    pub normal_curve_position_delta_threshold: f32,
    pub super_curve_position_delta_threshold: f32,
    pub z_speed_delta: f32,
}

impl PaddleImpactModifiers {
    pub fn starting() -> Self {
        PaddleImpactModifiers {
            normal_curve_scale: 6.0,
            super_curve_scale: 18.0,
            normal_curve_position_delta_threshold: 0.002,
            super_curve_position_delta_threshold: 0.006,
            z_speed_delta: 1.0,
        }
    }
}
