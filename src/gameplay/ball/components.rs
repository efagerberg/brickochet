use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct BallModifiers {
    pub base_radius: f32,
    pub base_velocity: Vec3,
}
impl BallModifiers {
    pub fn starting() -> Self {
        BallModifiers {
            base_radius: 0.75,
            base_velocity: Vec3::new(0.0, 0.0, 20.0),
        }
    }
}
