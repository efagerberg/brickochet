use bevy::prelude::*;

#[derive(Component)]
pub struct DepthLines;

#[derive(Component, PartialEq, Eq)]
pub enum Goal {
    Player,
    Enemy,
}
