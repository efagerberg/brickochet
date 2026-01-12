use bevy::prelude::*;

use crate::playfield;

pub fn spawn_bricks(
    mut commands: Commands,
    playfield: Res<playfield::resources::Playfield>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_position = -playfield.half_depth + Vector3::Z;
    let area = (playfield.half_width * 2.0) + (playfield.half_height * 2.0);
    let max_size = Vec2::new(6, 3);
    let min_size = Vec2::new(1, 1);
}
