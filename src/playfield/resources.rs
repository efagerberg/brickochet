use bevy::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct Playfield {
    pub half_size: Vec3,
    pub wall_line_default_color: LinearRgba,
    pub wall_line_highlight_color: LinearRgba,
}
