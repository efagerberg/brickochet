use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: u8,
    pub current: u8,
}

#[derive(Component)]
pub struct HealthColors {
    pub max: LinearRgba,
    pub min: LinearRgba,
}
