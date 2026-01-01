use bevy::prelude::*;

#[derive(Resource)]
pub struct Playfield {
    pub half_width: f32,
    pub half_height: f32,
    pub half_depth: f32,
}
