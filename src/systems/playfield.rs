use crate::{
    components::{self, ball},
    resources,
};
use bevy::prelude::*;

pub fn highlight_depth_lines(
    ball: Single<&Transform, With<components::ball::Ball>>,
    lines: Query<
        (&Transform, &mut MeshMaterial3d<StandardMaterial>),
        With<components::playfield::DepthLine>,
    >,
    playfield: Res<resources::playfield::Playfield>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball_z = ball.translation.z;
    let max_distance = ball::RADIUS * 4.0;

    for (line_transform, mat_handle) in lines {
        if let Some(mat) = materials.get_mut(&*mat_handle) {
            let distance = (line_transform.translation.z - ball_z).abs();
            let t = (max_distance - distance).clamp(0.0, 1.0); // 0 if far, 1 if very close
            let base_color = &playfield.wall_line_default_color;
            let highlight_color = &playfield.wall_line_highlight_color;
            mat.base_color = Color::mix(base_color, highlight_color, t);
        }
    }
}
