use bevy::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct Playfield {
    pub wall_line_default_color: LinearRgba,
    pub wall_line_highlight_color: LinearRgba,
    pub ball_distance_near_color: LinearRgba,
    pub ball_distance_far_color: LinearRgba,
    pub brick_size: Vec3,
}
