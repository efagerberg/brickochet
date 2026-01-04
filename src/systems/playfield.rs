use crate::{
    components::{self, ball},
    resources,
};
use bevy::prelude::*;

pub fn highlight_depth_lines(
    ball: Single<&Transform, With<components::ball::Ball>>,
    lines: Query<
        (&Transform, &mut MeshMaterial3d<StandardMaterial>),
        With<components::playfield::DepthLines>,
    >,
    playfield: Res<resources::playfield::Playfield>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball_z = ball.translation.z;
    // 2 ball diameters distance away, increase for smoothing animation, decrease
    // to make animation more choppy
    let max_distance = 2.0 * ball::RADIUS * 2.0;
    let base_color = &playfield.wall_line_default_color;
    let highlight_color = &playfield.wall_line_highlight_color;

    for (line_transform, mat_handle) in lines {
        if let Some(mat) = materials.get_mut(&*mat_handle) {
            let distance = (line_transform.translation.z - ball_z).abs();
            let t = (max_distance - distance).clamp(0.0, 1.0); // 0 if far, 1 if very closet);
            let new_color = LinearRgba::mix(base_color, highlight_color, t);
            if mat.emissive != new_color {
                mat.emissive = new_color;
            }
        }
    }
}
