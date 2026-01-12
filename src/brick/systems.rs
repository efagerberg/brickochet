use bevy::prelude::*;

use crate::playfield;

pub fn spawn_bricks(
    mut commands: Commands,
    playfield: Res<playfield::resources::Playfield>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_position = -playfield.aabb.half_extents.z + Vector3::Z;
    let area = (playfield.aabb.half_extents.x * 2.0) + (playfield.aabb.half_extents.y * 2.0);
    let max_size = Vec2::new(6, 3);
    let min_size = Vec2::new(1, 1);
}
