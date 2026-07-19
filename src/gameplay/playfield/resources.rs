use bevy::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct Playfield {
    pub ball_distance_near_color: LinearRgba,
    pub ball_distance_far_color: LinearRgba,
    pub brick_size: Vec3,
    pub half_size: Vec3,
}
