use bevy::prelude::*;

#[derive(Component)]
pub struct DepthLine;

#[derive(Component, PartialEq, Eq, Copy, Clone)]
pub enum Goal {
    Player,
    Enemy,
}
