use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: u8,
    pub current: u8,
}
