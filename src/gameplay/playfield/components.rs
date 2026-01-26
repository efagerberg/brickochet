use bevy::prelude::*;

#[derive(Component)]
pub struct DepthLines;

#[derive(Component, PartialEq, Eq, Copy, Clone)]
pub enum Goal {
    Player,
    Enemy,
}
