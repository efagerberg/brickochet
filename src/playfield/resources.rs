use bevy::prelude::*;

use crate::physics;

#[derive(Resource, Clone, Default)]
pub struct Playfield {
    pub bounds: physics::components::BoundingCuboid,
    pub wall_line_default_color: LinearRgba,
    pub wall_line_highlight_color: LinearRgba,
}
