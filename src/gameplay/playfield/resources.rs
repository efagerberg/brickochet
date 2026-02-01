use bevy::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct Playfield {
    pub wall_line_default_color: LinearRgba,
    pub wall_line_highlight_color: LinearRgba,
    pub brick_size: Vec3,
}
