use bevy::prelude::*;

#[derive(Resource)]
pub struct Playfield {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
