use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Clone, Copy)]
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

#[derive(Component, Default)]
pub struct PaddleImpactModifiers {
    pub normal_curve_scale: f32,
    pub super_curve_scale: f32,
    pub normal_curve_position_delta_threshold: f32,
    pub super_curve_position_delta_threshold: f32,
    pub contact_z_speed_increase: f32,
}

impl PaddleImpactModifiers {
    pub fn starting() -> Self {
        return PaddleImpactModifiers {
            normal_curve_scale: 6.0,
            super_curve_scale: 18.0,
            normal_curve_position_delta_threshold: 9.0,
            super_curve_position_delta_threshold: 4.0,
            contact_z_speed_increase: 1.0,
        };
    }
}
